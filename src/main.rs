extern crate rand;

mod service;

use std::alloc::System;
use kafka::producer::{Producer, Record, RequiredAcks};
use serde::{Deserialize, Serialize};
use kafka::error::Error as KafkaError;
use serde_json;
use chrono::{DateTime, Local, TimeZone, Utc};
use std::thread;
use threadpool::ThreadPool;
use std::str;
use std::str::Utf8Error;
use std::thread::Thread;
use std::time;
use rand::{Rng, thread_rng};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use service::agent::*;
use service::perf::*;
use service::tsts::*;

fn main() {
    env_logger::init();

    // println!("{}" , Local::now().timestamp()+"000".to_string());

    thread::spawn(|| {

        let broker : String = "localhost:9093".to_owned();

        let agent_info_topic = "agentinfo";

        if let Err(e1) =  agent_info_sender(agent_info_topic, vec![broker]) {
            println!("Failed Real Time messages: {}", e1);
        }
    });

    thread::spawn(|| {
        let broker : String = "localhost:9093".to_owned();
        let test_topic = "test";

        if let Err(e2) =  test_sender(test_topic, vec![broker]) {
            println!("Failed test messages: {}", e2);
        }
    });

    let broker : String = "localhost:9093".to_owned();
    let real_time_topic = "realtimeperf";

    if let Err(e3) =  real_time_sender(real_time_topic, vec![broker]) {
        println!("Failed Agent Info messages: {}", e3);
    }
}
