use crate::view::{View, extent};
use std::{cell::RefCell, rc::Rc};
use bitflags;
use thiserror::Error;

/// Shedules changes to the children list
#[derive(Clone, Debug)]
pub struct ChildrenScheduler {
    /// The shedule queue, the first element is the first operation which takes effect
    queue: Vec<ChildrenScheduleOperation>,
    /// Flags to tell whether there is operations on the queue or on one of the childrens queues
    flags: ChildrenCheduleFlags,
    /// The parent scheduler
    parent_scheduler: Option<Rc<RefCell<ChildrenScheduler>>>,
    /// The extent controllers for all of the children
    children_extent_controllers: Vec<Rc<RefCell<extent::ExtentController>>>,
}

impl ChildrenScheduler {
    /// Pushes an operation to the queue
    /// 
    /// # Parameters
    /// 
    /// operation: The operation to push
    pub fn push_operation(&mut self, operation: ChildrenScheduleOperation) -> Result<(), ValidateError> {
        // Make sure the operation is valid
        operation.validate(&self.children_extent_controllers)?;

        // Update extents
        operation.update(&mut self.children_extent_controllers);

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
    /// parent_scheduler: The scheduler for the parent view, None if it is the root
    pub(super) fn new(parent_scheduler: Option<Rc<RefCell<ChildrenScheduler>>>) -> Self {
        let queue = Vec::new();
        let flags = ChildrenCheduleFlags::NONE;

        Self { queue, flags, parent_scheduler, children_extent_controllers: Vec::new() }
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
    Push(Box<View>),
    /// Insert a view into the children list at some position
    Insert(Box<View>, usize),
    /// Move a view from one position to another, the first usize is the original position, the second is the new location.
    /// The new location is calculated after the old one is removed
    Move(usize, usize),
    /// Deletes a view from a specified position
    Delete(usize),
}

impl ChildrenScheduleOperation {
    /// Resolves the operation
    fn resolve(self, children: &mut Vec<Box<View>>) {
        match self {
            // Push the view onto the end
            Self::Push(view) => children.push(view),

            // Insert the view at the given position
            Self::Insert(view, pos) => children.insert(pos, view),

            // Move a view from one position to another
            Self::Move(from, to) => {
                let view = children.remove(from);
                children.insert(to, view);
            },

            // Delete a view
            Self::Delete(pos) => {
                children.remove(pos);
            }
        }
    }

    /// Validates the operation
    fn validate(&self, children_extent: &Vec<Rc<RefCell<extent::ExtentController>>>) -> Result<(), ValidateError> {
        match &self {
            // Just validate the view itself
            Self::Push(view) => view.validate(children_extent)?,

            // Just validate itself and that the pos is valid
            Self::Insert(view, pos) => {
                // Make sure index is not too large
                if *pos > children_extent.len() {
                    return Err(ValidateError::OutOfRange(*pos, children_extent.len()));
                }

                // Validate itself
                view.validate(&children_extent[..*pos])?
            }

            // Validate itself if moved forward, validate the next does not use prev if from=0 and validate views which may get invalid id's
            Self::Move(from, to) => {
                // Make sure indices are not too large
                if *to >= children_extent.len() {
                    return Err(ValidateError::OutOfRange(*to, children_extent.len() - 1));
                }
                if *from >= children_extent.len() {
                    return Err(ValidateError::InvalidPos(*from, children_extent.len()));
                }

                // If moved back, validate itself
                if *to < *from {
                    if *to == 0 && children_extent[*from].borrow().check_prev() {
                        return Err(ValidateError::NoPrev(*from));
                    }
                    if children_extent[*from].borrow().check_id_range(*to..*from) {
                        return Err(ValidateError::InvalidId(*from));
                    }
                } else if *to > *from { // If it is move forward, validat all other views
                    if *from == 0 && children_extent[1].borrow().check_prev() {
                        return Err(ValidateError::NoPrev(1));
                    }
                    for (pos, sibling) in children_extent[*from + 1..*to].iter().enumerate() {
                        if sibling.borrow().check_id(*from) {
                            return Err(ValidateError::InvalidId(pos + *from + 1));
                        }
                    }
                }
            },

            // Validate all views after the deleted views
            Self::Delete(pos) => {
                // Make sure the position is valid
                if *pos >= children_extent.len() {
                    return Err(ValidateError::InvalidPos(*pos, children_extent.len()));
                }

                // Check all other views
                for (check_pos, sibling) in children_extent[*pos + 1..].iter().enumerate() {
                    if sibling.borrow().check_id(*pos) {
                        return Err(ValidateError::InvalidId(check_pos + *pos + 1));
                    }
                }
            }
        }

        Ok(())
    }

    /// Update the extents
    fn update(&self, children_extent: &mut Vec<Rc<RefCell<extent::ExtentController>>>) {
        match &self {
            // Just append the new controller
            Self::Push(view) => children_extent.push(view.get_extent_controller()),

            // Update all id's after this point
            Self::Insert(view, pos) => {
                let iter_pos = if children_extent.len() == *pos {
                    *pos
                } else {
                    *pos + 1
                };
                for controller in children_extent[iter_pos..].iter_mut() {
                    controller.borrow_mut().update_insert(*pos);
                }
            }

            // Update all views after the smallest of from/to
            Self::Move(from, to) => {
                let min_pos = if *from < *to {
                    *from
                } else {
                    *to
                };
                for controller in children_extent[min_pos + 1..].iter_mut() {
                    controller.borrow_mut().update_move(*from, *to);
                }
            },

            // Update all views after the deleted view
            Self::Delete(pos) => {
                for controller in children_extent[*pos + 1..].iter_mut() {
                    controller.borrow_mut().update_delete(*pos);
                }
            }
        }
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

#[derive(Error, Debug, Clone)]
pub enum ValidateError {
    #[error("The view cannot be inserted into position {:?} of view list with length {:?}", .0, .1)]
    OutOfRange(usize, usize),
    #[error("Position {:?} of view list with length {:?} does not exist", .0, .1)]
    InvalidPos(usize, usize),
    #[error("The operation cannot be applied because the view at position {:?} with a reference to previous view, would be moved to the back", .0)]
    NoPrev(usize),
    #[error("The operation cannot be applied because the view at position {:?} would be moved behind one of its references", .0)]
    InvalidId(usize),
    #[error("The new view cannot be inserted because it is invalid: {:?}", .0)]
    InvalidNew(extent::ValidateError),
}

impl From<extent::ValidateError> for ValidateError {
    fn from(err: extent::ValidateError) -> ValidateError {
        ValidateError::InvalidNew(err)
    }
}