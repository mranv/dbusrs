use std::{error::Error, future::pending};
use zbus::{connection, interface};

struct Greeter;

#[interface(name = "com.atcults.anubhav")]
impl Greeter {
    fn say_hello(&self, name: &str) -> String {
        format!("Hello {}!", name)
    }
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let greeter = Greeter;
    let _conn = connection::Builder::session()?
        .name("com.atcults.anubhav")?
        .serve_at("/com/atcults/anubhav", greeter)?
        .build()
        .await?;

    // Do other things or go to wait forever
    pending::<()>().await;

    Ok(())
}
