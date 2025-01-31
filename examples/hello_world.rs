use std::time::Duration;

async fn this_errors(msg: &str) -> std::io::Result<()> {
    println!("{msg}");
    Err(std::io::Error::other("uh oh!"))
}

#[tokio::main()]
async fn main() {
    let hello = tokio::spawn(async move {
        mulligan::until_ok()
            .stop_after(5)
            .max_delay(Duration::from_secs(3))
            .full_jitter()
            .exponential(Duration::from_secs(1))
            .retry(|| async { this_errors("hello").await })
            .await
    });
    let world = tokio::spawn(async move {
        mulligan::until(|r| r.is_ok())
            .stop_after(10)
            .full_jitter()
            .fixed(Duration::from_secs(1))
            .retry(|| async { this_errors("world").await })
            .await
    });

    let _ = hello.await;
    let _ = world.await;
}
