#![allow(dead_code)]
use std::fs::read_to_string;

pub fn get_input(day: usize) -> String {
    read_to_string(format!("inputs/day{}.txt", day)).unwrap()
}

pub fn get_small(day: usize) -> String {
    read_to_string(format!("inputs/day{}_small.txt", day)).unwrap()
}

pub fn get_smalln(day: usize, n: usize) -> String {
    read_to_string(format!("inputs/day{}_small{}.txt", day, n)).unwrap()
}

pub type Point = (isize, isize);
