
use std::rc::Rc;

use datetime::LocalDateTime;

use super::client::Client;

pub struct MetaData{
    date_time: LocalDateTime,
    client: Rc<Client>
}
