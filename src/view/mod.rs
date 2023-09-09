mod extent;
mod children;
//mod update;

pub use extent::{Ratio, update::{ExtentUpdate, ExtentUpdateSingle, ExtentUpdateType, ExtentStretch, ExtentLocate, SizeType, PositionType, AnchorPoint, RefView}};
pub use children::scheduler::{ChildrenScheduler, ChildrenScheduleOperation};
//pub use update::ViewUpdater;

use thiserror::Error;
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
        let locate = ExtentLocate { pos: PositionType::Set(0.0), size: SizeType::Set(1.0) };
        let update_single = ExtentUpdateSingle { extent_type: ExtentUpdateType::Locate(locate), scale_rel: 1.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 };
        let update_info = ExtentUpdate { x: update_single, y: update_single };

        Self::new(update_info, None)
    }

    /// Creates a new view
    /// 
    /// # Parameters
    /// 
    /// update_info: The extent update info decsribing how the extent is constructed
    /// 
    /// parent_scheduler: The scheduler for the parent view, None if it is the root
    pub fn new(update_info: ExtentUpdate, parent_scheduler: Option<Rc<RefCell<ChildrenScheduler>>>) -> Box<Self> {
        let children = children::Children::new(parent_scheduler);
        let extent = extent::Extent::new(update_info);
        let sibling_id = None;

        Box::new(Self { children, extent, sibling_id })
    }

    /// Gets the children scheduler
    pub fn get_children_scheduler(&self) -> Rc<RefCell<ChildrenScheduler>> {
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
    pub(crate) fn validate(&self, siblings: &[Box<View>]) -> Result<(), ChildValidateError> {
        self.extent.get_update_info().borrow().validate(siblings)
    }
}


#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum ChildValidateError {
    #[error("A sibling ID of {:?} is too large, it must be smaller than {:?}", .0, .1)]
    WrongId(usize, usize),
    #[error("Reference to previous sibling is invalid when view is the first child")]
    NoPrev,
    #[error("Position of {:?} is invalid, it must be smaller than or equal to {:?}", .0, .1)]
    LargePos(usize, usize),
    #[error("This view cannot be deleted because sibling {:?} is referencing this view by ID", .0)]
    SiblingDependenceId(usize),
    #[error("This view cannot be deleted because the next sibling is referencing this view and there is no prev sibling")]
    SiblingDependencePrev,
    #[error("An extent cannot use aspect mode for both dimensions")]
    BothRatio,
}

#[cfg(test)]
mod tests {

}