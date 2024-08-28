use std::{fmt::Display, rc::Rc};
use std::fmt;

use super::{client::Client, metadata::MetaData};


pub struct Message {
    meta: MetaData,
    msg: String,
    msg_id: u64
}

impl Message {
    pub fn new( from: Rc<Client>, msg: String, msg_id: u64 ) -> Message {
        let meta = MetaData::new(from);
        Message {
            meta,
            msg,
            msg_id
        }
    }
}

impl Display for Message{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "-----Message: {}-----\n{}\nMessage:{}\n----------------", self.msg_id,self.meta,self.msg)
    }
}