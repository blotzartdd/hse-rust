#![forbid(unsafe_code)]

use std::collections::VecDeque;

#[derive(Default)]
pub struct MinMaxQueue {
    queue: VecDeque<i32>,
    min_deque: VecDeque<i32>,
    max_deque: VecDeque<i32>,
}

impl MinMaxQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            min_deque: VecDeque::new(),
            max_deque: VecDeque::new(),
        }
    }

    pub fn push(&mut self, value: i32) {
        self.queue.push_back(value);

        while !self.min_deque.is_empty() && self.min_deque.back().copied().unwrap() > value {
            self.min_deque.pop_back();
        }

        while !self.max_deque.is_empty() && self.max_deque.back().copied().unwrap() < value {
            self.max_deque.pop_back();
        }

        self.min_deque.push_back(value);
        self.max_deque.push_back(value);
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.queue.front() == self.min_deque.front() {
            self.min_deque.pop_front();
        }

        if self.queue.front() == self.max_deque.front() {
            self.max_deque.pop_front();
        }

        self.queue.pop_front()
    }

    pub fn first(&self) -> Option<i32> {
        self.queue.front().copied()
    }

    pub fn last(&self) -> Option<i32> {
        self.queue.back().copied()
    }

    pub fn min(&self) -> Option<i32> {
        self.min_deque.front().copied()
    }

    pub fn max(&self) -> Option<i32> {
        self.max_deque.front().copied()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
