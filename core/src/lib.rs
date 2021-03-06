//! Core of OpenGame Application

pub mod command;
pub mod error;
pub mod os;
pub mod package;
pub mod settings;
pub mod utils;

#[derive(Default, Debug)]
pub struct OpenGame {}
