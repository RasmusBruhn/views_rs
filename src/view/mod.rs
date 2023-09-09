mod extent;
//mod update;

pub use extent::{Ratio, update::{ExtentUpdate, ExtentUpdateSingle, ExtentUpdateType, ExtentStretch, ExtentLocate, SizeType, PositionType, AnchorPoint, RefView}};
//pub use update::ViewUpdater;

use thiserror::Error;

/*
use bitflags;

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct UpdateFlags: u8 {
        const NONE = 0x00;
        const UPDATE_EXTENT_SELF = 0x01;
        const UPDATE_EXTENT_CHILD = 0x02;
    }
}*/

/// A view struct containing all the information of a single view
#[derive(Clone, Debug)]
pub struct View { 
    /// A vector containing all of the children of the view, children cannot be removed only added
    children: Vec<Box<View>>,
    /// The current extent of the view, this is relative to its parent, (0, 0) to (1, 1) would be the entire parent extent
    extent: extent::Extent,
} // TODO: Add back in the sibling id

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

        Self::new(update_info)
    }

    /// Creates a new view
    /// 
    /// # Parameters
    /// 
    /// update_info: The extent update info decsribing how the extent is constructed
    pub fn new(update_info: ExtentUpdate) -> Box<Self> {
        let children = Vec::new();
        let extent = extent::Extent::new(update_info);

        Box::new(Self { children, extent })
    }

    /// Pushes a new child into the end of the children list.
    /// 
    /// Returns an error if it could not validate the child.
    /// 
    /// # Parameters
    /// 
    /// child: The child view to add
    pub fn push_child(&mut self, child: Box<View>) -> Result<(), ChildValidateError> {
        // Validate that the child is valid
        child.validate(&self.children)?;

        // Push it
        self.children.push(child);
        
        Ok(())
    }

    /// Inserts a new child into some position of the children list.
    /// 
    /// Returns an error if it could not validate the child.
    /// 
    /// # Parameters
    /// 
    /// child: The child view to add
    /// 
    /// pos: The position to add it to, this must be smaller or equal to the number to children currently in the parent view
    /// 
    /// # Errors
    pub fn insert_child(&mut self, child: Box<View>, pos: usize) -> Result<(), ChildValidateError> {
        // Make sure the position is valid
        if pos > self.children.len() {
            return Err(ChildValidateError::LargePos(pos, self.children.len()));
        }
        
        // Validate that the child is valid
        child.validate(&self.children[..pos])?;

        // Update indices of the other children


        Ok(())
    }

    /// Deletes a child from the children list if it does not invalidate any references.
    /// 
    ///  Returns an error if any other sibling will be invalidated by removing this one.
    /// 
    /// # Parameters
    /// 
    /// pos: The position of the child to delete
    pub fn delete_child(&mut self, _pos: usize) {
        todo!();
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