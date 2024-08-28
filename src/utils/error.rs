use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};


pub type ResultAx<T> = core::result::Result<T, ServiceBusError>;

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





impl IntoResponse for ServiceBusError {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {:#?}", "INTO_RES -- SB Err", self);

        match self {
            ServiceBusError::QueueOverflow => (
                StatusCode::NOT_ACCEPTABLE,
                format!("The queue is overflow the max number of items is: {}", isize::MAX),
            )
                .into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response(),
        }
    }
}
