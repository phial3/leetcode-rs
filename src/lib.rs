//! Solution for `LeetCode` with `rust-lang`  
//! 
//! TODO: docs

pub mod list_node;
pub mod problems;
pub mod solutions;

pub trait LeetCode {
    fn difficulty(&self) -> Difficulty;
    fn status(&self) -> Status;
    fn tags(&self) -> Vec<String>;
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

pub enum Status {
    Todo,
    Solved,
    Attempted
}

