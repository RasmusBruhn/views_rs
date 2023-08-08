use super::{View, ExtentUpdate, UpdateFlags};
use std::iter;

/// Struct for allowing external users to update views
pub struct ViewUpdater<'a> {
    /// The view to allow updating
    view: &'a mut View,
    /// The view updater of the parent
    parent: Option<&'a mut ViewUpdater<'a>>,
}

impl<'a> ViewUpdater<'a> {
    pub fn get_extent_update_info(&'a mut self) -> &'a mut ExtentUpdate {
        &mut self.view.extent.update_info
    }

    fn set_update_child(&mut self) {
        // If flag was not already set, set it and for it's parent
        if !self.view.update_flags.contains(UpdateFlags::UPDATE_EXTENT_CHILD) {
            self.view.update_flags.extend(UpdateFlags::UPDATE_EXTENT_CHILD);
            match self.parent.as_mut() {
                // If there is a parent pass up the information
                Some(parent) => parent.set_update_child(),

                // Do nothing if it is the root
                None => (),
            }
        }
    }
}