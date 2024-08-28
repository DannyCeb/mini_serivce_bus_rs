
use std::{ fmt::Display, rc::Rc};

use datetime::{Instant, LocalDateTime};

use super::client::Client;

pub struct MetaData{
    date_time: LocalDateTime,
    client: Rc<Client>
}

impl MetaData {
    pub fn new(client: Rc<Client>) -> MetaData {
        let date_time = LocalDateTime::from_instant(Instant::now());
        MetaData {
            date_time,
            client
        }
    }
}

impl Display for MetaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "---MetaData---\nDate_time: {:?}\nClient:{:?}", self.date_time, self.client)
    }
}
