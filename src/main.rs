use std::sync::{Arc, Mutex};
use std::thread::{self};
mod chapter3;
mod chapter4;
mod lbs;
mod saftey_example;

fn main() {
    chapter4::problem_4_1();
    saftey_example::problem_3_8();
}
