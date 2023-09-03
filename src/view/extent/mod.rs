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
    /// The ratio of w to h in absolute size on the screen, None if either w or h are <= 0
    ratio: Option<Ratio>,
    /// The update information
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
        let ratio = Ratio::new(1.0, 1.0);
        Self { x: 0.0, y: 0.0, w: 1.0, h: 1.0, update_info , ratio}
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
    pub(super) fn validate(&mut self, siblings: &[Box<View>]) -> Result<(), ChildValidateError> {
        self.update_info.validate(siblings)
    }

    /// Updates the extent
    /// 
    /// # Parameters
    /// 
    /// siblings: All the older siblings
    pub(super) fn update(&mut self, siblings: &[Box<View>], parent_ratio: Ratio) {
        // Get the new extent
        (self.x, self.y, self.w, self.h) = self.update_info.get(siblings, parent_ratio);
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

}
