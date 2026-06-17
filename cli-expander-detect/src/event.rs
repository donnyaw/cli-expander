#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    KeyPress { code: u16, key: String },
    KeyRelease { code: u16, key: String },
}
