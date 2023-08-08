mod extent;
mod update;

pub use extent::update::{ExtentUpdate, ExtentUpdateSingle, ExtentUpdateType, ExtentStretch, ExtentLocate, SizeType, PositionType, AnchorPoint, RefView};
pub use update::ViewUpdater;

use bitflags;

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct UpdateFlags: u8 {
        const NONE = 0x00;
        const UPDATE_EXTENT_SELF = 0x01;
        const UPDATE_EXTENT_CHILD = 0x02;
    }
}

/// A view struct containing all the information of a single view
#[derive(Clone, Debug)]
pub struct View { 
    /// A vector containing all of the children of the view, children cannot be removed only added
    children: Vec<Box<View>>,
    /// The current extent of the view, this is relative to its parent, (0, 0) to (1, 1) would be the entire parent extent
    extent: extent::Extent,
    /// The position of the view in the parent child list, None if it does not have a parent
    sibling_id: Option<usize>,
    /// Flags to determine if the view needs any updating
    update_flags: UpdateFlags,
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
    pub fn new_root() -> Self {
        // Create the extent update to cover the entire screen
        let locate = ExtentLocate { pos: PositionType::Set(0.0), size: SizeType::Set(1.0) };
        let update_single = ExtentUpdateSingle { extent_type: ExtentUpdateType::Locate(locate), scale_rel: 1.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 };
        let update_info = ExtentUpdate { x: update_single, y: update_single };

        Self::new(update_info)
    }

    /// Creates a new view
    /// 
    /// # Examples
    /// 
    /// Create view which is half the size of its parent and located in the middle
    /// 
    /// ```
    /// use views::view::{View, update};
    /// 
    /// let locate = update::ExtentLocate { pos: update::PositionType::Set(0.25), size: update::SizeType::Set(0.5) };
    /// let update_single = update::ExtentUpdateSingle { extent_type: update::ExtentUpdateType::Locate(locate), scale_rel: 1.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 };
    /// let update_info = update::ExtentUpdate { x: update_single, y: update_single };
    /// 
    /// let view = View::new(update_info);
    /// ```
    pub fn new(update_info: ExtentUpdate) -> Self {
        let children = Vec::new();
        let extent = extent::Extent::new(update_info);
        let sibling_id = None;
        let update_flags = UpdateFlags::NONE;

        Self {children, extent, sibling_id, update_flags}
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_root() {
        let root = View::new_root();

        let locate = ExtentLocate { pos: PositionType::Set(0.0), size: SizeType::Set(1.0) };
        let update_single = ExtentUpdateSingle { extent_type: ExtentUpdateType::Locate(locate), scale_rel: 1.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 };
        let update_info = ExtentUpdate { x: update_single, y: update_single };
        let extent = extent::Extent::new(update_info);

        assert_eq!(0, root.children.len());
        assert_eq!(extent, root.extent);
    }
}