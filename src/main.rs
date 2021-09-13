use std::sync::Arc;

use hyper::{Body, Method, Request, Response, Server, StatusCode, service::{make_service_fn, service_fn}};
use tokio::{fs::File, sync::Mutex};
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::msqw::MSQW;

mod msqw;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

static NOTFOUND: &[u8] = b"Not Found";

async fn bits_api(
    pool: Arc<Mutex<MSQW>>,
    req: Request<Body>
) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => simple_file_send("static/index.html").await,
        (&Method::GET, "/index.js") => simple_file_send("static/index.js").await,
        (&Method::GET, "/bits") => api_get_bits(pool).await,
        (&Method::POST, "/zero") => api_post_zero(pool).await,
        (&Method::POST, "/one") => api_post_one(pool).await,
        _ => {
            // Return 404 not found response.
            Ok(not_found())
        }
    }
}

async fn api_get_bits(pool: Arc<Mutex<MSQW>>) -> Result<Response<Body>> {
    // always return a u32
    let bits = format!("{:032b}", pool.lock().await.update());

    let response = Response::builder()
        .status(200)
        .header("Content-Type", "application/text")
        .body(bits.into())
        .unwrap();

    Ok(response)
}

async fn api_post_zero(pool: Arc<Mutex<MSQW>>) -> Result<Response<Body>> {
    pool.lock().await.add(false);
    Ok(Response::new("OK".into()))
}

async fn api_post_one(pool: Arc<Mutex<MSQW>>) -> Result<Response<Body>> {
    pool.lock().await.add(true);
    Ok(Response::new("OK".into()))
}

async fn simple_file_send(filename: &str) -> Result<Response<Body>> {
    // Serve a file by asynchronously reading it by chunks using tokio-util crate.

    if let Ok(file) = File::open(filename).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        return Ok(Response::new(body));
    }

    Ok(not_found())
}

fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOTFOUND.into())
        .unwrap()
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let pool =  Arc::new(Mutex::new(MSQW::new()));
    let pool = &*Box::leak(Box::new(pool));

    let service = make_service_fn(
        move |_| async move { 
            Ok::<_, hyper::Error>(service_fn(move |req| bits_api(pool.clone(), req)))
        });
    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}