use crate::task::Task;

pub struct List<'a> {
    head: Option<&'a Node<'a>>,
    count: u32,
}

impl<'a> List<'a> {
    pub fn new() -> Self {
        List {
            head: None,
            count: 0,
        }
    }
}

pub struct Node<'a> {
    pub task: Task<'a>,
    pub next: Option<&'a Node<'a>>,
}

impl<'a> Node<'a> {
    pub fn new(task: Task<'a>) -> Self {
        Node {
            task: task,
            next: None,
        }
    }
}

