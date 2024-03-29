use async_std::task;
use zbus::{Connection, Result, proxy};

#[proxy(
    interface = "com.atcults.anubhav",
    default_service = "com.atcults.anubhav",
    default_path = "/com/atcults/anubhav"
)]
trait MyGreeter {
    fn say_hello(&self, name: &str) -> Result<String>;
}

async fn client_task(name: &'static str) -> Result<()> {
    let connection = Connection::session().await?;
    let proxy = MyGreeterProxy::new(&connection).await?;
    
    loop {
        // Await the result of say_hello
        let reply = proxy.say_hello(name).await?;
        println!("{}", reply);
        // Sleep for a short duration before sending the next message
        task::sleep(std::time::Duration::from_secs(1)).await;
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    let clients = vec!["Alice", "Bob", "Charlie"];

    let client_tasks = clients.into_iter().map(|name| {
        task::spawn(async move {
            if let Err(e) = client_task(name).await {
                eprintln!("Client error: {}", e);
            }
        })
    });

    for task in client_tasks {
        task.await;
    }

    Ok(())
}
