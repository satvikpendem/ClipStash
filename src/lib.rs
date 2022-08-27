pub mod lib {
    pub mod data {
        pub mod model;
    }
    pub mod domain {
        pub mod clip;
        pub mod time;
    }
    pub mod service;
    pub mod web;
}

pub use lib::domain::{
    clip::{field::shortcode::ShortCode, ClipError},
    time::Time,
};
