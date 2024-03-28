use dbus::{blocking::Connection, message::Message, strings::RequestName};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the session bus (could also be `Connection::new_system()` for system bus)
    let c = Connection::new_session()?;

    // Request a name on the bus
    c.request_name("com.example.SampleService", RequestName::ReplaceExisting, 0)?;

    // Create a new message handler
    let handler = Box::new(|m, _| {
        let response = Message::method_return();
        let response = response.append1("Pong".to_string());
        Ok(vec![response])
    });

    // Add the message handler
    c.register_object_path("/com/example/SampleObject")?;
    c.register_interface(
        "/com/example/SampleObject",
        "com.example.SampleInterface",
    )?;
    c.add_method_handler("Ping", "/com/example/SampleObject", "com.example.SampleInterface", handler)?;

    println!("Server is listening for incoming messages...");

    // Event loop
    loop {
        c.handle_incoming(None)?;
    }
}