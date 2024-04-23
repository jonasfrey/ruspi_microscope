#[tokio::main]
async fn main() {
    use warp::Filter;

    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes)
        .tls()
        // RSA
        .cert_path("cert.pem")
        .key_path("key.pem")
        // ECC
        // .cert_path("examples/tls/cert.ecc.pem")
        // .key_path("examples/tls/key.ecc")
        // .run(([127, 0, 0, 1], 3030))
        .run(([192,168,1,105], 3030))
        .await;
}
