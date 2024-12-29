use std::collections::VecDeque;
use std::sync::RwLock;
use once_cell::sync::OnceCell;

pub enum Work {
    Download { url: String },
    Analyse { path: String },
    Quit,
}

pub struct WorkQueue {
    queue: VecDeque<Work>,
}

impl WorkQueue {
    pub fn new() -> WorkQueue {
        WorkQueue {
            queue: VecDeque::new(),
        }
    }

    pub fn get() -> std::sync::RwLockWriteGuard<'static, WorkQueue> {
        WORK_QUEUE.get_or_init(|| RwLock::new(WorkQueue::new()));
        WORK_QUEUE.get().unwrap().write().unwrap()
    }

    pub fn push(&mut self, work: Work) {
        self.queue.push_back(work);
    }

    pub fn pop(&mut self) -> Option<Work> {
        self.queue.pop_front()
    }

    pub fn quit(&mut self) {
        self.queue.push_front(Work::Quit);
    }
}

pub static WORK_QUEUE: OnceCell<RwLock<WorkQueue>> = OnceCell::new();