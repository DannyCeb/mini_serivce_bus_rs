
#[derive(Debug)]
pub struct Client {
    id: u64,
    display_name: String
}

impl Client {
    pub fn new(id: u64, display_name: String) -> Self {
        Client{
            id,
            display_name
        }
    }
}