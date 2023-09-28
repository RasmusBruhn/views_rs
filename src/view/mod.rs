pub mod extent;
pub mod children;

use std::{cell::RefCell, rc::Rc};

/// A view struct containing all the information of a single view
#[derive(Clone, Debug)]
pub struct View { 
    /// A vector containing all of the children of the view, children cannot be removed only added
    children: children::Children,
    /// The current extent of the view, this is relative to its parent, (0, 0) to (1, 1) would be the entire parent extent
    extent: extent::Extent,
    /// The position of this view in the parents child list
    sibling_id: Option<usize>,
}

impl View {
    /// Creates a new root view.
    /// This view will cover the entire window exactly
    /// 
    /// # Examples
    /// 
    /// ```
    /// use views::view::View;
    /// 
    /// let root = View::new_root();
    /// ```
    pub fn new_root() -> Box<Self> {
        // Create the extent update to cover the entire screen
        let locate = extent::ExtentLocate { pos: extent::PositionType::Set(0.0), size: extent::SizeType::Set(1.0) };
        let update_single = extent::ExtentUpdateSingle { extent_type: extent::ExtentUpdateType::Locate(locate), scale_rel: 1.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 };
        let update_info = extent::ExtentUpdate { x: update_single, y: update_single };

        Self::new(update_info, None)
    }

    /// Creates a new view
    /// 
    /// # Parameters
    /// 
    /// update_info: The extent update info decsribing how the extent is constructed
    /// 
    /// parent_scheduler: The scheduler for the parent view, None if it is the root
    pub fn new(update_info: extent::ExtentUpdate, parent_scheduler: Option<Rc<RefCell<children::ChildrenScheduler>>>) -> Box<Self> {
        let children = children::Children::new(parent_scheduler);
        let extent = extent::Extent::new(update_info);
        let sibling_id = None;

        Box::new(Self { children, extent, sibling_id })
    }

    /// Gets the extent controller
    pub fn get_extent_controller(&self) -> Rc<RefCell<extent::ExtentController>> {
        self.extent.get_controller()
    }

    /// Gets the children scheduler
    pub fn get_children_scheduler(&self) -> Rc<RefCell<children::ChildrenScheduler>> {
        self.children.get_scheduler()
    }

    /// Resolves all updates to the children
    pub(crate) fn resolve_children(&mut self) {
        self.children.resolve()
    }

    /// Updates the list extent and graphics, should be run once in the event loop once all the user events are handled
    pub(crate) fn update(&self) {
        todo!();
    }

    /// Validates the view
    pub(crate) fn validate(&self, siblings: &[Rc<RefCell<extent::ExtentController>>]) -> Result<(), extent::ValidateError> {
        self.extent.borrow_controller().validate(siblings)
    }
}

#[cfg(test)]
mod tests {

}