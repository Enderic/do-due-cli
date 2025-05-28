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

    // Insert by alphabetical order of task names
    pub fn insert(&mut self, node: &'a mut Node<'a>) {

        let new_task_name = node.task.name;

        let mut trav = self.head;
        
        // Check for head, if it exists, & if new node should be inserted before
        match trav {
            Some(n) => {
                if new_task_name < n.task.name {
                    node.next = Some(n);
                    self.head = Some(node);
                    return
                }
            },
            None => {
                self.head = Some(node);
                return
            },
        }
    }

    pub fn delete() {

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