use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

pub mod src;
use src::data;
async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // 处理请求的逻辑
    let full_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
    src::analyze_post_body(full_body);
    let response = Response::new(Body::from("Hello, World!"));
    Ok(response)
}

#[tokio::main]
async fn main() {
    data::transfer_data();
    // 创建一个绑定到本地地址的 TCP 监听器
    let addr = SocketAddr::from(([127, 0, 0, 1], 5701));

    // 创建一个服务，用于处理每个连接
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

    // 创建一个 HTTP 服务器，并绑定到监听器上
    let server = Server::bind(&addr).serve(make_svc);

    // 启动服务器并等待它处理连接
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
