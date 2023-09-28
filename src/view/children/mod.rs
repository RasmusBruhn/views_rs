mod scheduler;

use super::View;
use std::{cell::RefCell, rc::Rc};

pub use scheduler::{ChildrenScheduler, ValidateError};

/// All data related to children including the list and the sheduler for changing the list
#[derive(Clone, Debug)]
pub(super) struct Children {
    /// The list of current children
    list: Vec<Box<View>>,
    /// The sheduler for the list
    scheduler: Rc<RefCell<scheduler::ChildrenScheduler>>,
}

impl Children {
    /// Creates a new children struct
    /// 
    /// # Parameters
    /// 
    /// parent_scheduler: The scheduler for the parent view, None if it is the root
    pub(super) fn new(parent_scheduler: Option<Rc<RefCell<scheduler::ChildrenScheduler>>>) -> Self {
        let list = Vec::new();
        let scheduler = Rc::new(RefCell::new(scheduler::ChildrenScheduler::new(parent_scheduler)));

        Self { list, scheduler }
    }

    /// Gets an instance of the children scheduler
    pub(super) fn get_scheduler(&self) -> Rc<RefCell<scheduler::ChildrenScheduler>> {
        Rc::clone(&self.scheduler)
    }
    
    /// Resolves all the scheduled changes
    pub(super) fn resolve(&mut self) {
        self.scheduler.borrow_mut().resolve(&mut self.list);
    }
}