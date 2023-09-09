use super::{View, ExtentUpdate, ChildValidateError};

/// Struct for allowing external users to update views
pub struct ViewUpdater<'a> {
    /// The view to allow updating
    view: &'a mut View,
    /// The view updater of the parent
    parent: Option<&'a mut ViewUpdater<'a>>,
}

impl<'a> ViewUpdater<'a> {
    /// Creates a new view updater
    pub(super) fn new(view: &'a mut View, parent: Option<&'a mut ViewUpdater<'a>>) -> Self {
        Self { view, parent }
    }

    /// Gets a mutable reference to the extent update info for the view.
    /// This will trigger the view extent to be updated durin g the next update call
    pub fn get_extent_update_info(&'a mut self) -> &'a mut ExtentUpdate {
        // Set the update flag for itself and up the parent chain
        self.view.update_flags.extend(UpdateFlags::UPDATE_EXTENT_SELF);
        self.set_extent_update_parent();

        // Return the extent update info
        &mut self.view.extent.update_info
    }

    /// Sets the flag for updating the extent of a child, for the parent, if it exists
    fn set_extent_update_parent(&mut self) {
        // Make sure there is a parent
        if let Some(parent) = &mut self.parent {
            // If flag was not already set, set it and for it's parent
            if !self.view.update_flags.contains(UpdateFlags::UPDATE_EXTENT_CHILD) {
                self.view.update_flags.extend(UpdateFlags::UPDATE_EXTENT_CHILD);
                parent.set_extent_update_parent();
            }
        }
    }

    /// Deletes the view. This will consume the view.
    /// 
    /// Returns an error if it was unable to delete.
    /// 
    /// # Errors
    /// 
    /// ChildValidateError::SiblingDependenceId: If another sibling references this one by ID
    /// 
    /// ChildValidateError::SiblingDependencePrev: If pos = 0 and the next sibling is referencing the previous
    pub fn delete(self) -> Result<(), ChildValidateError> {
        todo!();
    }
}