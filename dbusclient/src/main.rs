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
    let reply = match proxy.say_hello(name).await {
        Ok(reply) => reply,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(e.into());
        }
    };
    println!("{}", reply);
    Ok(())
}

#[async_std::main]
async fn main() -> Result<()> {
    let clients = vec![
    "Alice", "Bob", "Charlie", "David", "Emma", "Frank", "Grace", "Henry", "Isabella", "Jack",
    "Katherine", "Liam", "Mia", "Noah", "Olivia", "Peter", "Quinn", "Rachel", "Samuel", "Taylor",
    "Ursula", "Victor", "Wendy", "Xavier", "Yvonne", "Zachary", "Abigail", "Benjamin", "Catherine",
    "Daniel", "Eleanor", "Finn", "Gabriella", "Harry", "Isabelle", "James", "Kaitlyn", "Landon",
    "Madison", "Nathan", "Oliver", "Penelope", "Quentin", "Rebecca", "Sarah", "Thomas", "Uma",
    "Vincent", "Willa", "Xander", "Yasmine", "Zara",
    ];

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
