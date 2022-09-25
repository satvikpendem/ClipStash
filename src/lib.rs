pub mod lib {
    pub mod data;
    pub mod domain {
        pub mod clip;
        pub mod time;
    }
    pub mod service;
    pub mod web;
}

pub use lib::{
    data::DataError,
    domain::{
        clip::{field::shortcode::ShortCode, ClipError},
        time::Time,
    },
};
