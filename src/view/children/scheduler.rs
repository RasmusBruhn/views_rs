use crate::view::{View, ChildValidateError};
use std::{cell::RefCell, rc::Rc};
use bitflags;

/// Shedules changes to the children list
#[derive(Clone, Debug)]
pub struct ChildrenScheduler {
    /// The shedule queue, the first element is the first operation which takes effect
    queue: Vec<ChildrenScheduleOperation>,
    /// Flags to tell whether there is operations on the queue or on one of the childrens queues
    flags: ChildrenCheduleFlags,
    /// The parent scheduler
    parent_scheduler: Option<Rc<RefCell<ChildrenScheduler>>>,
}

impl ChildrenScheduler {
    /// Pushes an operation to the queue
    /// 
    /// # Parameters
    /// 
    /// operation: The operation to push
    pub fn push_operation(&mut self, operation: ChildrenScheduleOperation) -> Result<(), ChildValidateError> {
        // Make sure the operation is valid
        todo!();

        // Push it to the queue
        self.queue.push(operation);

        // Shedule the update
        if let Some(parent_scheduler) = &mut self.parent_scheduler {
            parent_scheduler.borrow_mut().child_received_item();
        }

        Ok(())
    }

    /// Creates a new children scheduler
    /// 
    /// # Parameters
    /// 
    /// parent: The scheduler for the parent view, None if it is the root
    pub(super) fn new(parent_scheduler: Option<Rc<RefCell<ChildrenScheduler>>>) -> Self {
        let queue = Vec::new();
        let flags = ChildrenCheduleFlags::NONE;

        Self { queue, flags, parent_scheduler }
    }

    // resolves all the operations and clears the queue
    pub(super) fn resolve(&mut self, children: &mut Vec<Box<View>>) {
        // Resolve all the operations
        for operation in self.queue.drain(..) {
            operation.resolve(children);
        }

        // Resolve operations for all children
        if self.flags.contains(ChildrenCheduleFlags::CHILDREN_QUEUE_ITEM) {
            for child in children {
                child.resolve_children();
            }

            // Clear the flag
            self.flags.remove(ChildrenCheduleFlags::CHILDREN_QUEUE_ITEM)
        }
    }
    
    // Adds the flag to tell one of the children has an operation
    fn child_received_item(&mut self) {
        // Make sure the flag was not already there
        if !self.flags.contains(ChildrenCheduleFlags::CHILDREN_QUEUE_ITEM) {
            // Apply the flag
            self.flags.insert(ChildrenCheduleFlags::CHILDREN_QUEUE_ITEM);

            // Apply it to the parent
            if let Some(parent_scheduler) = &self.parent_scheduler {
                parent_scheduler.borrow_mut().child_received_item();
            }
        }
    }
}

/// The different operations to do on the child list
#[derive(Clone, Debug)]
pub enum ChildrenScheduleOperation {
    /// Push a view onto the end of the children list
    Push(View),
    /// Insert a view into the children list at some position
    Insert(View, usize),
    /// Move a view from one position to another, the first usize is the original position, the second is the new location.
    /// The new location is calculated before the old one is removed which means that both original and original+1 are the same position
    Move(usize, usize),
    /// Deletes a view from a specified position
    Delete(usize),
}

impl ChildrenScheduleOperation {
    /// Resolves the operation
    fn resolve(self, children: &mut Vec<Box<View>>) {
        todo!();
    }

    /// Validates the operation
    fn validate(&self, children: &mut Vec<Box<View>>) -> Result<(), ChildValidateError> {
        todo!();
    }
}

bitflags::bitflags! {
    /// Flags for determining whether there are items on the queue
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct ChildrenCheduleFlags: u8 {
        /// No items
        const NONE = 0;
        /// There are items on one of the childrens queues
        const CHILDREN_QUEUE_ITEM = 1 << 0;
    }
}
