extern crate dbus;

use dbus::{Connection, BusType};

fn main() {
    // Connect to the session bus
    let c = Connection::get_private(BusType::Session).unwrap();

    // Create a proxy for the remote object
    let proxy = c.with_proxy("com.example.SampleService", "/com/example/SampleObject", Duration::from_millis(5000));

    // Call the remote method
    let response: &str = proxy.method_call("com.example.SampleInterface", "Ping", ()).unwrap();
    
    println!("Response: {}", response);
}
