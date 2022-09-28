use std::fmt;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};
use itertools_num::linspace;
use serde::{Serialize, Deserialize};
use crate::gui::{print_to_console, Print, update_in_console};

const BUF_LEN: usize = 1024;
const READ_HEADER_LEN: usize = 19;

#[derive(Clone, Debug)]
pub enum SerialDirection {
    SEND,
    RECEIVE,
}

impl fmt::Display for SerialDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SerialDirection::SEND => write!(f, "SEND"),
            SerialDirection::RECEIVE => write!(f, "RECV"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Packet {
    pub time: u128,
    pub direction: SerialDirection,
    pub payload: String,
}

impl Default for Packet {
    fn default() -> Packet {
        return Packet {
            time: 0,
            direction: SerialDirection::SEND,
            payload: "".to_string(),
        };
    }
}

#[derive(Clone, Debug)]
pub struct DataContainer {
    pub time: Vec<u128>,
    pub dataset: Vec<Vec<f32>>,
    pub raw_traffic: Vec<Packet>,
}

impl Default for DataContainer {
    fn default() -> DataContainer {
        return DataContainer {
            time: vec![],
            dataset: vec![vec![]],
            raw_traffic: vec![],
        };
    }
}