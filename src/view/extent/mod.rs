pub mod update;
use super::{View, ChildValidateError};

/// Defines the extent of a view
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Extent {
    /// The x-position of the upper left corner 
    x: f32, 
    /// The y-position of the upper left corner 
    y: f32, 
    /// The width
    w: f32, 
    /// The height
    h: f32,
    // The update information
    pub(super) update_info: update::ExtentUpdate,
}

impl Extent {
    /// Creates a new extent, the size defaults to (x, y, w, h) = (0, 0, 1, 1) 
    /// but this should not be used before updating the extent with Extent.update()
    /// 
    /// # Parameters
    /// 
    /// update_info: The information on how to update the extent
    pub(super) fn new(update_info: update::ExtentUpdate) -> Self {
        Self { x: 0.0, y: 0.0, w: 1.0, h: 1.0, update_info }
    }

    /// Tests whether a point (x, y) is within the extent.
    /// Including (x1, y1) but excluding (x2, y2)
    /// 
    /// # Parameters
    /// 
    /// x: The x-position of the point
    /// 
    /// y: The y-position of the point
    fn contained(&self, x: f32, y: f32) -> bool {
        x >= self.x && y >= self.y && x < self.x + self.w && y < self.y + self.h
    }

    /// Checks whether the update info has any invalid references. Returns an error in case of an invalid reference.
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    /// 
    /// # Errors
    /// 
    /// ChildValidateError::WrongId: If a reference to a sibling by ID is invalid, it is invalid if the ID is larger than the number of children
    /// 
    /// ChildValidateError::NoPrev: If a reference to the previous sibling is used but this is the first child
    pub(super) fn validate(&self, siblings: &[Box<View>]) -> Result<(), ChildValidateError> {
        self.update_info.validate(siblings)
    }

    /// Updates the extent
    /// 
    /// # Parameters
    /// 
    /// siblings: All the older siblings
    pub(super) fn update(&mut self, siblings: &[Box<View>]) {
        // Get the new extent
        (self.x, self.y, self.w, self.h) = self.update_info.get(siblings);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contained_inside() {
        let update_type = update::ExtentUpdateType::Stretch(update::ExtentStretch { pos1: update::PositionType::Set(0.0), pos2: update::PositionType::Set(1.0) });
        let update_single = update::ExtentUpdateSingle { extent_type: update_type, scale_rel: 1.0, scale_abs: 0.0, offset_abs: 0.0, offset_rel: 0.0 };
        let extent = Extent::new(update::ExtentUpdate { x: update_single, y: update_single });

        assert!(extent.contained(0.5, 0.5));
    }

    #[test]
    fn contained_outside() {
        let update_type = update::ExtentUpdateType::Stretch(update::ExtentStretch { pos1: update::PositionType::Set(0.0), pos2: update::PositionType::Set(1.0) });
        let update_single = update::ExtentUpdateSingle { extent_type: update_type, scale_rel: 1.0, scale_abs: 0.0, offset_abs: 0.0, offset_rel: 0.0 };
        let extent = Extent::new(update::ExtentUpdate { x: update_single, y: update_single });

        assert!(!extent.contained(2.0, 0.0));
    }

    #[test]
    fn contained_edge() {
        let update_type = update::ExtentUpdateType::Stretch(update::ExtentStretch { pos1: update::PositionType::Set(0.0), pos2: update::PositionType::Set(1.0) });
        let update_single = update::ExtentUpdateSingle { extent_type: update_type, scale_rel: 1.0, scale_abs: 0.0, offset_abs: 0.0, offset_rel: 0.0 };
        let extent = Extent::new(update::ExtentUpdate { x: update_single, y: update_single });

        assert!(extent.contained(0.0, 0.5));
        assert!(extent.contained(0.5, 0.0));
        assert!(!extent.contained(1.0, 0.5));
        assert!(!extent.contained(0.5, 1.0));
    }
}
