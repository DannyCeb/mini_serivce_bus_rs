use mini_serivce_bus_rs::{entities::{client::Client, message::Message, queue::Queue}, handler::network_manager::routes_queue};
use tokio::net::TcpListener;

use axum::{routing::get, Router};

/* 
fn main() {
    println!("Hello, world!");

    let mut clients: Vec<Rc<Client>> = vec!();

    clients.push( Rc::new(Client::new(1, "Danny".to_string())) );
    clients.push( Rc::new(Client::new(2, "Daniela".to_string())) );

    let mut miniq = Queue::new();

    miniq.push_message(Message::new(clients[0].clone(), "Hola".to_string(), 1)).unwrap();

    let msg_recuperado = miniq.pop_message().unwrap();

    print!("{}",msg_recuperado);

}*/

#[tokio::main]
async fn main() {
    let router = routes_queue();
    let addr = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("->> Listening on {:?}\n", addr);

    axum::serve(addr,router).await.unwrap();

    //Ok(())
}

