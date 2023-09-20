use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::thread;
use std::time::{Duration, SystemTime};
use tokio::runtime::Runtime;
mod io;
mod mytype;
mod produce;
mod src;
mod test;
async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // 处理请求的逻辑
    let full_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
    if let Err(err) = src::analyze_post_body(full_body).await {
        println!("error occured when handle_request: {}", err);
    }
    let response = Response::new(Body::from("Hello, World!"));
    Ok(response)
}

#[tokio::main]
async fn main() {
    let handle = thread::spawn(|| {
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 5701));
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(err) = server.await {
        println!("server error: {}", err);
    }
    let _ = handle.join().unwrap();
}
