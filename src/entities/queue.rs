use crate::{entities::entities_model::QueueResponder, utils::error::{ResultAx, ServiceBusError}};

use super::{entities_model::PushRequest, message::Message};

use axum::{response::IntoResponse, Json};


pub struct Queue {
    queue: Vec<Message>
}

impl Queue {

    pub fn new() -> Queue {
        Queue{
            queue: Vec::new()
        }
    }

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

pub async fn handler_push(Json(push_request): Json<PushRequest>) -> ResultAx<impl IntoResponse> {
    println!("-->> Handler Push");

    //Ok(Json(FibResponse::new(n, result)))
    Ok(QueueResponder::respond_json(QueueResponder::Push { message: push_request.message, id: 7 }))
}