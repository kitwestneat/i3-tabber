extern crate i3ipc;
use i3ipc::I3Connection;
use i3ipc::I3EventListener;
use i3ipc::Subscription;
use i3ipc::event::Event;

fn get_window_list() {
    let mut connection = I3Connection::connect().unwrap();

    let window_list = connection.get_tree().unwrap();

    println!("window_list: {:?}", window_list);
}

fn main() {
    get_window_list();
    let mut lru = create_lru();

    /*
    // establish connection.
    let mut listener = I3EventListener::connect().unwrap();

    // subscribe to a couple events.
    let subs = [Subscription::Mode, Subscription::Binding];
    listener.subscribe(&subs).unwrap();

    // handle them
    for event in listener.listen() {
        match event.unwrap() {
            Event::ModeEvent(e) => println!("new mode: {}", e.change),
            Event::BindingEvent(e) => println!("user input triggered command: {}", e.binding.command),
            _ => unreachable!()
        }
    }
    */
}
