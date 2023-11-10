use crossbeam_channel::{bounded, Receiver, Sender};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Request, Response, Server};
use lazy_static::lazy_static;
use std::net::SocketAddr;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, SystemTime};
use tokio::runtime::Runtime;
mod file_io;
mod mytype;
mod produce;
mod src;
mod test;
mod web_io;
lazy_static! {
    static ref CHANNEL: Mutex<(Sender<Request<Body>>, Receiver<Request<Body>>)> = {
        let (sender, receiver) = bounded(100);
        Mutex::new((sender, receiver))
    };
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Error> {
    // 处理请求的逻辑
    let full_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let sender = CHANNEL.lock().unwrap().0.clone();
    sender.send(Request::new(Body::from(full_body))).unwrap();
    let response = Response::new(Body::from("Hello, World!"));
    Ok(response)
}

#[tokio::main]
async fn main() {
    let timer_ticker = thread::spawn(|| {
        loop {
            let now = SystemTime::now();
            let current_time = now
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let target_time = current_time - (current_time % 86400) + 20 * 3600; // 4 * 3600 表示四点的秒数
            let sleep_duration = if current_time < target_time {
                Duration::from_secs(target_time - current_time)
            } else {
                Duration::from_secs(target_time + 86400 - current_time)
            };
            println!(
                "before work:{} {} {:?}",
                current_time, target_time, sleep_duration
            );
            thread::sleep(sleep_duration);
            println!("work!!!");
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                if let Err(err) = src::daily_work().await {
                    println!("error when work daily:{}", err);
                }
            });
        }
    });

    let analyzer = thread::spawn(|| {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            loop {
                let receiver = CHANNEL.lock().unwrap().1.clone();
                let req = receiver.recv().unwrap();
                tokio::spawn(async move {
                    src::analyze_post_body(hyper::body::to_bytes(req.into_body()).await.unwrap())
                        .await;
                });
            }
        });
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 5701));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Error>(service_fn(handle_request)) });
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(err) = server.await {
        println!("server error: {}", err);
    }
    let _ = timer_ticker.join().unwrap();
    let _ = analyzer.join().unwrap();
}
