use std::fmt;

#[derive(Debug)]
pub enum ServiceBusError {
    QueueOverflow,
    Generic
}


impl fmt::Display for ServiceBusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceBusError::QueueOverflow => {
                write!( f, "The queue is overflow" )
            },
            _ => {
                write!(f, "Generic Error")
            }
        }
        
    }
}

impl std::error::Error for ServiceBusError {
    fn description(&self) -> &str {
        match self {
            ServiceBusError::QueueOverflow => {
                "The queue is overflow"
            },
            Self::Generic => {
                "Generic Error"
            }
        }
    }
}
