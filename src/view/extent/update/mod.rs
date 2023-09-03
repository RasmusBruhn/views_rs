use super::{View, ChildValidateError, Ratio};

mod validate;
mod get;
mod update;

/// Defines how the entire extent should update
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExtentUpdate {
    /// Defines how the x-dimension should update
    pub x: ExtentUpdateSingle,
    /// Defines how the y-dimension should update
    pub y: ExtentUpdateSingle,
    /// The dimension which is fixed (not using ratio) (x if both are fixed)
    fixed: Dim,
}

impl ExtentUpdate {
    /// Creates a new extent update info
    /// 
    /// # Parameters
    /// 
    /// x: The info for updating x
    /// 
    /// y: The info for updating y
    pub fn new(x: ExtentUpdateSingle, y: ExtentUpdateSingle) -> Self {
        Self { x, y, fixed: Dim::X }
    }
}

/// Defines how a single dimension should update
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExtentUpdateSingle {
    /// Defines how the base extent should update
    pub extent_type: ExtentUpdateType,
    /// Scales the size by a relative amount.
    /// scale_rel = 1 keeps the size the same
    /// 
    /// Scaling is applied after offsets and relative scaling is applied before absolute scaling
    pub scale_rel: f32,
    /// Scales the size by an absolute amount.
    /// scale_abs = 0 keeps the size the same
    /// 
    /// Scaling is applied after offsets and absolute scaling is applied after relative scaling
    pub scale_abs: f32,
    /// Offsets the base extent relative to the size, 
    /// offset_rel = 1 moves the extent exactly one size in the positive direction.
    /// 
    /// Offsets are applied before scaling
    pub offset_rel: f32,
    /// Offsets the base extent by a set amount.
    /// 
    /// Offsets are applied before scaling
    pub offset_abs: f32,    
}

/// The different types of methods to update the extent
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExtentUpdateType {
    /// The extent is updated by stretching it between two points
    Stretch(ExtentStretch),
    /// The extent is updated by giving it a position and a size
    Locate(ExtentLocate),
    /// The extent is updated using a fixed extent
    Ratio(ExtentRatio),
}

/// Defines how to update the extent when a fixed ratio between w and h is used
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExtentRatio {
    /// The position of the extent
    pub pos: PositionType,
}

/// Defines how to update the extent in Locate mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExtentLocate {
    /// Defines how the position is updated
    pub pos: PositionType,
    /// Defines how the size is updated
    pub size: SizeType,
}

/// The different ways to update the size
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SizeType {
    /// Update size by stretching between two points
    Stretch(ExtentStretch),
    /// Update the size by making it relative to another size
    Relative(RefView),
    /// Update the size by giving it a static value
    Set(f32),
}

/// Defines how to stretch between two points
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExtentStretch {
    /// Defines the position at the lowest coordinate
    pub pos1: PositionType,
    /// Defines the position at the highest coordinate
    pub pos2: PositionType,
}

/// The different ways to get a position
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PositionType {
    /// Get the position relative from another view
    Anchor(AnchorPoint),
    /// Use a static position
    Set(f32),
}

/// Defines how to get a point from another view
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnchorPoint {
    /// The view to get it from
    pub ref_view: RefView,
    // Where on the view to anchor to, 0 is the lowest coordinate side and 1 is the highest, everything else is a linear interpolation
    pub ref_point: f32,
}

/// The different ways to reference another view
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RefView {
    /// Use the previous sibling view, useful for lists
    Prev,
    /// Use the Id of a sibling which is older than this one
    Id(usize),
}

/// Describes what dimension to get the coordinate from
#[derive(Clone, Copy, Debug, PartialEq)]
enum Dim {
    /// The x direction
    X,
    /// The y-direction
    Y,
}

impl Dim {
    /// Get the correct dimension data from the view
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to get data for
    /// 
    /// view: The view to extract the data from
    fn get_from_view(&self, view: &View) -> (f32, f32) {
        match *self {
            // Get the x-dimension
            Self::X => (view.extent.x, view.extent.w),

            // Get the y-dimension
            Self::Y => (view.extent.y, view.extent.h),
        }
    }
}

#[cfg(test)]
mod tests {

}