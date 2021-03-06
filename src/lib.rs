extern crate bytes;
extern crate chrono;
extern crate futures;
extern crate futures_cpupool;
extern crate handlebars;
extern crate hostname;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

pub mod configuration;
mod debug;
pub mod service;
pub mod templates;
