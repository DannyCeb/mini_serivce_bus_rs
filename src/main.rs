use std::rc::Rc;

use mini_serivce_bus_rs::objects::{client::Client, message::Message, queue::Queue};

fn main() {
    println!("Hello, world!");

    let mut clients: Vec<Rc<Client>> = vec!();

    clients.push( Rc::new(Client::new(1, "Danny".to_string())) );
    clients.push( Rc::new(Client::new(2, "Daniela".to_string())) );

    let mut miniq = Queue::new();

    miniq.push_message(Message::new(clients[0].clone(), "Hola".to_string(), 1)).unwrap();

    let msg_recuperado = miniq.pop_message().unwrap();

    print!("{}",msg_recuperado);

}
