use zbus::{Connection, Result, proxy};

#[proxy(
    interface = "com.atcults.anubhav",
    default_service = "com.atcults.anubhav",
    default_path = "/com/atcults/anubhav"
)]
trait MyGreeter {
    async fn say_hello(&self, name: &str) -> Result<String>;
}

// Although we use `async-std` here, you can use any async runtime of choice.
#[async_std::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;

    // `proxy` macro creates `MyGreeterProxy` based on `Notifications` trait.
    let proxy = MyGreeterProxy::new(&connection).await?;
    let reply = proxy.say_hello("Anubhav").await?;
    println!("{reply}");

    Ok(())
}