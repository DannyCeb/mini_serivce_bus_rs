use std::io::Error;

use super::message::{self, Message};


struct Queue {
    queue: Vec<Message>
}

 impl Queue {
    pub fn push_message(&mut self, msg: Message) -> Result<bool,Error>{
        self.queue.push( msg );

        Ok(true)
    }
}