pub mod play;
pub mod stream;
pub mod pipe;

#[derive(Debug, PartialEq)]
pub enum Plugins {
    Play,
    Stream,
    Pipe,
}
