use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::any().map(|| "Hello, world!");

    println!("Warp, http://127.0.0.1:8080/");
    warp::serve(hello).run(([127, 0, 0, 1], 8080)).await;
}
