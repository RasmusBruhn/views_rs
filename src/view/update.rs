use super::{View, ExtentUpdate};

/// Struct for allowing external users to update views
pub struct ViewUpdater<'a> {
    /// The view to allow updating
    view: &'a mut View,
    /// The view updater of the parent
    parent: Option<&'a mut ViewUpdater<'a>>,
}

impl<'a> ViewUpdater<'a> {
    pub fn get_extent_updater(&mut self) -> ExtentUpdater<'a> {
        let view = &mut self;
        let extent = &mut self.view.extent.update_info;

        ExtentUpdater { view, extent }
    }

    fn update_extent(&mut self) {
        todo!();
    }
}

pub struct ExtentUpdater<'a> {
    view: &'a mut ViewUpdater<'a>,
    pub extent: &'a mut ExtentUpdate,
}

impl<'a> Drop for ExtentUpdater<'a> {
    fn drop(&mut self) {
        self.view.update_extent();
    }
}