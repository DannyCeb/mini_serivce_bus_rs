use super::{message::Message, error::ServiceBusError};


struct Queue {
    queue: Vec<Message>
}

impl Queue {
    pub fn push_message(&mut self, msg: Message) -> Result<bool, ServiceBusError>{
        
        match self.queue.len() as isize {
            isize::MAX => Err( ServiceBusError::QueueOverflow ),
            _ => {
                self.queue.push( msg );
                Ok(true)
            }
        }

        
    }

    pub fn pop_message(&mut self) -> Option<Message> {
        self.queue.pop()
    }
}