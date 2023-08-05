mod extent;
pub use extent::update;

use self::update::ExtentUpdateSingle;

/// A view struct containing all the information of a single view
#[derive(Clone, Debug)]
pub struct View { 
    /// A vector containing all of the children of the view, children cannot be removed only added
    children: Vec<Box<View>>,
    /// The current extent of the view, this is relative to its parent, (0, 0) to (1, 1) would be the entire parent extent
    extent: extent::Extent,
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
        let locate = update::ExtentLocate { pos: update::PositionType::Set(0.0), size: update::SizeType::Set(1.0) };
        let update_single = update::ExtentUpdateSingle { extent_type: update::ExtentUpdateType::Locate(locate), scale_rel: 1.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 };
        let update_info = update::ExtentUpdate { x: update_single, y: update_single };

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
    pub fn new(update_info: update::ExtentUpdate) -> Self {
        let children = Vec::new();
        let extent = extent::Extent::new(update_info);

        Self {children, extent}
    }

    /// Updates the extent of the view
    /// 
    /// # Parameters
    /// 
    /// siblings: All the older siblings
    fn update_extent(&mut self, siblings: &[Box<View>]) {
        self.extent.update(siblings);
    }

    fn update_graphics() {
        todo!();
    }

    fn render() {
        todo!();
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_root() {
        let root = View::new_root();

        let locate = update::ExtentLocate { pos: update::PositionType::Set(0.0), size: update::SizeType::Set(1.0) };
        let update_single = ExtentUpdateSingle { extent_type: update::ExtentUpdateType::Locate(locate), scale_rel: 1.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 };
        let update_info = update::ExtentUpdate { x: update_single, y: update_single };
        let extent = extent::Extent::new(update_info);

        assert_eq!(0, root.children.len());
        assert_eq!(extent, root.extent);
    }
}