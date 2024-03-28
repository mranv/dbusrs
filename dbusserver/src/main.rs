extern crate dbus;

use dbus::{Connection, BusType, NameFlag, ConnectionItem};
use std::sync::Arc;
use dbus::tree::{Factory, MethodErr, MTFn, MTStatic, Signal, EmitsChangedSignal};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

fn main() {
    // Connect to the session bus (could also be `Connection::new_system()` for system bus)
    let c = Connection::get_private(BusType::Session).unwrap();

    // Request a name on the bus
    c.register_name("com.example.SampleService", NameFlag::ReplaceExisting as u32).unwrap();

    // Create a tree
    let f = Factory::new_fn::<()>();
    let t = f.tree(()).add(f.object_path("/com/example/SampleObject", ()).introspectable().add(
        f.interface("com.example.SampleInterface", ()).add_m(
            f.method("Ping", (), move |m| {
                Ok(vec![m.msg.method_return().append("Pong")])
            }).outarg::<&str,_>("response"),
        ),
    ));

    // Export the tree on the connection
    t.set_registered(&c, true).unwrap();

    println!("Server is listening for incoming messages...");

    // Event loop
    loop {
        match c.incoming(1000).next() {
            Some(msg) => {
                c.process(msg);
            }
            None => {}
        }
    }
}
