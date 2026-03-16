mod client;
mod error;
pub mod types;
mod api;
pub mod ws;

pub use client::Bitstamp;
pub use ws::BitstampEventStream;
pub use error::Error;

#[cfg(test)]
mod tests;
