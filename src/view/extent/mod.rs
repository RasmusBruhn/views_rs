mod update;
use std::{cell::{RefCell, Ref}, rc::Rc, ops::Range};

use super::View;

pub use update::{ExtentUpdate, ExtentUpdateSingle, ExtentUpdateType, ExtentRatio, ExtentLocate, SizeType, ExtentStretch, PositionType, AnchorPoint, RefView};
pub use update::ValidateError;

/// A container for the extent update info
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExtentController {
    /// The update info
    update_info: update::ExtentUpdate,
}

impl ExtentController {
    /// Creates a new extent update container
    /// 
    /// # Parameters
    /// 
    /// update_info: The information on how to update the extent
    fn new(update_info: update::ExtentUpdate) -> Self {
        Self { update_info }
    }

    /// Checks whether the update info has any invalid references. Returns an error in case of an invalid reference.
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    pub(crate) fn validate(&self, siblings: &[Rc<RefCell<ExtentController>>]) -> Result<(), update::ValidateError> {
        self.update_info.validate(siblings)
    }

    /// Checks if the range of ID's are being references
    /// 
    /// # Parameters
    /// 
    /// range: The range to check for
    pub(crate) fn check_id_range(&self, range: Range<usize>) -> bool {
        self.update_info.check_id_range(range)
    }

    /// Checks if the ID is being referenced
    /// 
    /// # Parameters
    /// 
    /// id: The ID to check
    pub(crate) fn check_id(&self, id: usize) -> bool {
        self.update_info.check_id(id)
    }

    /// Checks if this view references the previous sibling
    pub(crate) fn check_prev(&self) -> bool {
        self.update_info.check_prev()
    }

    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted into
    pub(crate) fn update_insert(&mut self, pos: usize) {
        self.update_info.update_insert(pos);
    }

    /// Updates possible references by ID on movement of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// from: The original position of the sibling
    /// 
    /// to: The new position of the sibling
    pub(crate) fn update_move(&mut self, from: usize, to: usize) {
        self.update_info.update_move(from, to);
    }

    /// Updates possible references by ID on deletion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was deleted from
    pub(crate) fn update_delete(&mut self, pos: usize) {
        self.update_info.update_delete(pos);
    }

    /// Gets the extent
    /// 
    /// # Parameters
    /// 
    /// siblings: All the older siblings
    fn get(&self, siblings: &[Box<View>], parent_ratio: Ratio) -> (f32, f32, f32, f32) {
        self.update_info.get(siblings, parent_ratio)
    }
}

/// Defines the extent of a view
#[derive(Clone, Debug, PartialEq)]
pub struct Extent {
    /// The x-position of the upper left corner 
    x: f32, 
    /// The y-position of the upper left corner 
    y: f32, 
    /// The width
    w: f32, 
    /// The height
    h: f32,
    /// The ratio of w to h in absolute size on the screen, None if either w or h are <= 0
    ratio: Option<Ratio>,
    /// The update information
    update_info: Rc<RefCell<ExtentController>>,
}

impl Extent {
    /// Retrieves an instance of the extent controller refcell.
    pub(super) fn get_controller(&self) -> Rc<RefCell<ExtentController>> {
        Rc::clone(&self.update_info)
    }

    /// Retrieves an instance of the borrowed controller refcell.
    pub(super) fn borrow_controller(&self) -> Ref<'_, ExtentController> {
        self.update_info.borrow()
    }
    
    /// Creates a new extent, the size defaults to (x, y, w, h) = (0, 0, 1, 1) 
    /// but this should not be used before updating the extent with Extent.update()
    /// 
    /// # Parameters
    /// 
    /// update_info: The information on how to update the extent
    pub(super) fn new(update_info: update::ExtentUpdate) -> Self {
        let ratio = Ratio::new(1.0, 1.0);
        let update_info = Rc::new(RefCell::new(ExtentController::new(update_info)));
        Self { x: 0.0, y: 0.0, w: 1.0, h: 1.0, update_info , ratio}
    }

    /// Updates the extent
    /// 
    /// # Parameters
    /// 
    /// siblings: All the older siblings
    pub(super) fn update(&mut self, siblings: &[Box<View>], parent_ratio: Ratio) {
        (self.x, self.y, self.w, self.h) = self.update_info.borrow().get(siblings, parent_ratio);
        self.ratio = Ratio::new(self.w, self.h);
    }
}

/// Defines a ratio, this is always positive
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ratio {
    /// The value of the ratio
    value: f32,
}

impl Ratio {
    /// Creates a new ratio (w / h) or None if it is invalid (w or h <= 0)
    /// 
    /// # Parameters
    /// 
    /// w: The width
    /// 
    /// h: The height
    pub fn new(w: f32, h: f32) -> Option<Self> {
        if w > 0.0 && h > 0.0 {
            Some(Self { value: w / h })
        } else {
            None
        }
    }

    /// Returns the value of the ratio of w/h
    pub fn get_x(&self) -> f32 {
        self.value
    }

    /// Returns the value of the ratio of h/w
    pub fn get_y(&self) -> f32 {
        1.0 / self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod ratio {
        use super::*;

        #[test]
        fn new() {
            let ratio_value = Ratio::new(2.0, 5.0).unwrap();
            assert_eq!(2.0 / 5.0, ratio_value.value);

            let ratio_no_w = Ratio::new(0.0, 5.0);
            assert!(ratio_no_w.is_none());

            let ratio_no_h = Ratio::new(2.0, 0.0);
            assert!(ratio_no_h.is_none());
        }

        #[test]
        fn get() {
            let ratio = Ratio::new(2.0, 5.0).unwrap();
            assert_eq!(2.0 / 5.0, ratio.get_x());
            assert_eq!(5.0 / 2.0, ratio.get_y());
        }
    }

    mod extent {
        #[test]
        fn new() {
            
        }
    }
}
