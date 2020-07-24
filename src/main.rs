mod system;

use crate::system::{ConsoleApp};

#[macro_use]
extern crate colour;

fn main() {
    ConsoleApp::new().run();
}
