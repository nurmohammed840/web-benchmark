#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let addr = "127.0.0.1:8080";
    println!("Tide, http://{}", addr);
    
    let mut app = tide::new();
    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.listen(addr).await?;
    Ok(())
}
