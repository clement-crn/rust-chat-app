use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a simple route that returns "Hello, world!".
    let hello = warp::path!("hello" / "world").map(|| warp::reply::html("Hello, world!"));

    warp::serve(hello).run(([0, 0, 0, 0], 8080)).await;
    // Start the server on port 8080 (you can change this to your desired port).
}
