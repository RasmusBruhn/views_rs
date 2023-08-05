use crate::view::View;

/// Defines how the entire extent should update
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExtentUpdate {
    /// Defines how the x-dimension should update
    pub x: ExtentUpdateSingle,
    // Defines how the y-dimension should update
    pub y: ExtentUpdateSingle,  
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
}

/// Defines how to update the exent in Locate mode
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
    /// The parent view, this always has the extent (x, y, w, h) = (0, 0, 1, 1)
    Parent,
    /// Use the previous sibling view, useful for lists
    Prev,
    /// Use the Id of a sibling which is older than this one
    Id(usize),
}

/// Describes what dimension to get the coordinate from
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dim {
    /// The x direction
    X,
    /// The y-direction
    Y,
}

impl ExtentUpdate {
    /// Tests wether the possible reference views exists
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    pub(super) fn validate(&self, siblings: &[Box<View>]) -> bool {
        // Make sure both x and y are valid
        self.x.validate(siblings) && self.y.validate(siblings)
    }

    /// Retrieves the extent
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// siblings: The list of older siblings
    pub(super) fn get(&self, siblings: &[Box<View>]) -> (f32, f32, f32, f32) {
        // Get the x and y components
        let x = self.x.get(Dim::X, siblings);
        let y = self.y.get(Dim::Y, siblings);

        (x.0, y.0, x.1, y.1)
    }
}

impl ExtentUpdateSingle {
    /// Tests wether the possible reference views exists
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    fn validate(&self, siblings: &[Box<View>]) -> bool {
        // Make sure the extent is valid
        self.extent_type.validate(siblings)
    }

    /// Retrieves the position and size
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// siblings: The list of older siblings
    fn get(&self, dim: Dim, siblings: &[Box<View>]) -> (f32, f32) {
        // Get the base position and size
        let (mut pos, mut size) = self.extent_type.get(dim, siblings);

        // Apply changes
        pos += self.offset_abs + self.offset_rel * size;
        size *= self.offset_rel;
        size += self.offset_abs;

        // Make sure size is not negative
        if size < 0.0 {
            size = 0.0;
        }

        (pos, size)
    }
}

impl ExtentUpdateType {
    /// Tests wether the possible reference views exists
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    fn validate(&self, siblings: &[Box<View>]) -> bool {
        match *self {
            // Make sure stretch mode is valid
            Self::Stretch(stretch) => stretch.validate(siblings),

            // Make sure locate mode is valid
            Self::Locate(locate) => locate.validate(siblings),
        }
    }

    /// Retrieves the position and size
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// siblings: The list of older siblings
    fn get(&self, dim: Dim, siblings: &[Box<View>]) -> (f32, f32) {
        match *self {
            // Get from the stretch method
            Self::Stretch(stretch) => stretch.get(dim, siblings),

            // Get from the locate method
            Self::Locate(locate) => locate.get(dim, siblings),
        }
    }
}

impl ExtentLocate {
    /// Tests wether the possible reference views exists
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    fn validate(&self, siblings: &[Box<View>]) -> bool {
        // Make sure position and size are valid
        self.pos.validate(siblings) && self.size.validate(siblings)
    }
    
    /// Retrieves the position and size
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// siblings: The list of older siblings
    fn get(&self, dim: Dim, siblings: &[Box<View>]) -> (f32, f32) {
        // Get the position and size
        let pos = self.pos.get(dim, siblings);
        let size = self.size.get(dim, siblings);

        (pos, size)
    }
}

impl SizeType {
    /// Tests wether the possible reference views exists
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    fn validate(&self, siblings: &[Box<View>]) -> bool {
        match *self {
            // Make sure possible references in the stretch are valid
            Self::Stretch(stretch) => stretch.validate(siblings),

            // Make sure reference is valid
            Self::Relative(ref_view) => ref_view.validate(siblings),

            // Set is always valid
            Self::Set(_) => true,
        }
    }

    /// Retrieves the size
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// siblings: The list of older siblings
    fn get(&self, dim: Dim, siblings: &[Box<View>]) -> f32 {
        match *self {
            // Use the size from a stretch
            Self::Stretch(stretch) => stretch.get(dim, siblings).1,

            // Get the size from another view
            Self::Relative(ref_view) => ref_view.get(dim, siblings).1,

            // Use a static size
            Self::Set(size) => size,
        }
    }
}

impl ExtentStretch {
    /// Tests wether the possible reference views exists
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    fn validate(&self, siblings: &[Box<View>]) -> bool {
        // Make sure both positions are valid
        self.pos1.validate(siblings) && self.pos2.validate(siblings)
    }

    /// Retrieves the position and size
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// siblings: The list of older siblings
    fn get(&self, dim: Dim, siblings: &[Box<View>]) -> (f32, f32) {
        // Get the two positions
        let pos1 = self.pos1.get(dim, siblings);
        let pos2 = self.pos2.get(dim, siblings);

        (pos1, pos2 - pos1)
    }
}

impl PositionType {
    /// Tests wether the possible reference views exists
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    fn validate(&self, siblings: &[Box<View>]) -> bool {
        match *self {
            // Make sure the anchor point is valid
            Self::Anchor(anchor) => anchor.validate(siblings),

            // Set is always valid
            Self::Set(_) => true,
        }
    }

    /// Retrieves the position
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// siblings: The list of older siblings
    fn get(&self, dim: Dim, siblings: &[Box<View>]) -> f32 {
        match *self {
            // Get from an anchor point
            Self::Anchor(anchor) => anchor.get(dim, siblings),

            // Get a static position
            Self::Set(pos) => pos,
        }
    }
}

impl AnchorPoint {
    /// Tests wether the reference view exists
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    fn validate(&self, siblings: &[Box<View>]) -> bool {
        // Make sure the reference is valid
        self.ref_view.validate(siblings)
    }

    /// Retrieves the anchor point
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// siblings: The list of older siblings
    fn get(&self, dim: Dim, siblings: &[Box<View>]) -> f32 {
        // Get the position and size
        let (pos, size) = self.ref_view.get(dim, siblings);

        // Get the correct position
        pos + self.ref_point * size
    }
}

impl RefView {
    /// Tests wether the reference view exists
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    fn validate(&self, siblings: &[Box<View>]) -> bool {
        match *self {
            // Make sure the index is within the sibling list
            Self::Id(index) => siblings.len() > index,

            // Make sure the is a sibling if it references the previous
            Self::Prev => siblings.len() > 0,

            // There is always a parent
            Self::Parent => true,
        }
    }

    /// Retrieves the position and size of the view in the given dimension
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// siblings: The list of older siblings
    fn get(&self, dim: Dim, siblings: &[Box<View>]) -> (f32, f32) {
        match *self {
            // Get size from the parent
            Self::Parent => (0.0, 1.0),

            // Get size from previous sibling
            Self::Prev => dim.get_from_view(siblings.last().unwrap()),

            // Get size from id
            Self::Id(n) => dim.get_from_view(siblings.get(n).unwrap()),
        }
    }
}

impl Dim {
    /// Get the correct dimension data from the view
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to get data for
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
    use super::*;

    #[test]
    fn extent_update_validate() {
        let view_list = Vec::new();
        let single_extent_success = ExtentUpdateSingle { extent_type: ExtentUpdateType::Stretch(ExtentStretch { pos1: PositionType::Set(0.0), pos2: PositionType::Set(0.0) }), scale_rel: 0.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 };
        let single_extent_fail = ExtentUpdateSingle { extent_type: ExtentUpdateType::Locate(ExtentLocate {pos: PositionType::Set(0.0), size: SizeType::Relative(RefView::Prev)}), scale_rel: 0.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 };

        assert!(ExtentUpdate { x: single_extent_success, y: single_extent_success }.validate(&view_list));
        assert!(!ExtentUpdate { x: single_extent_fail, y: single_extent_success }.validate(&view_list));
        assert!(!ExtentUpdate { x: single_extent_success, y: single_extent_fail }.validate(&view_list));
        assert!(!ExtentUpdate { x: single_extent_fail, y: single_extent_fail }.validate(&view_list));
    }

    #[test]
    fn extent_update_single_validate() {
        let view_list = Vec::new();

        assert!(ExtentUpdateSingle { extent_type: ExtentUpdateType::Stretch(ExtentStretch { pos1: PositionType::Set(0.0), pos2: PositionType::Set(0.0) }), scale_rel: 0.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 }.validate(&view_list));
        assert!(!ExtentUpdateSingle { extent_type: ExtentUpdateType::Locate(ExtentLocate {pos: PositionType::Set(0.0), size: SizeType::Relative(RefView::Prev)}), scale_rel: 0.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 }.validate(&view_list));
    }

    #[test]
    fn extent_update_type_validate() {
        let view_list = Vec::new();

        assert!(ExtentUpdateType::Stretch(ExtentStretch { pos1: PositionType::Set(0.0), pos2: PositionType::Set(0.0) }).validate(&view_list));
        assert!(!ExtentUpdateType::Stretch(ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), pos2: PositionType::Set(0.0) }).validate(&view_list));
        assert!(ExtentUpdateType::Locate(ExtentLocate {pos: PositionType::Set(0.0), size: SizeType::Set(0.0)}).validate(&view_list));
        assert!(!ExtentUpdateType::Locate(ExtentLocate {pos: PositionType::Set(0.0), size: SizeType::Relative(RefView::Prev)}).validate(&view_list));
    }

    #[test]
    fn extent_locate_validate() {
        let view_list = Vec::new();

        assert!(ExtentLocate {pos: PositionType::Set(0.0), size: SizeType::Set(0.0)}.validate(&view_list));
        assert!(!ExtentLocate {pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), size: SizeType::Set(0.0)}.validate(&view_list));
        assert!(!ExtentLocate {pos: PositionType::Set(0.0), size: SizeType::Relative(RefView::Prev)}.validate(&view_list));
        assert!(!ExtentLocate {pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), size: SizeType::Relative(RefView::Prev)}.validate(&view_list));
    }

    #[test]
    fn size_type_validate() {
        let view_list = Vec::new();

        assert!(SizeType::Stretch(ExtentStretch { pos1: PositionType::Set(0.0), pos2: PositionType::Set(0.0) }).validate(&view_list));
        assert!(!SizeType::Stretch(ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), pos2: PositionType::Set(0.0) }).validate(&view_list));
        assert!(SizeType::Relative(RefView::Parent).validate(&view_list));
        assert!(!SizeType::Relative(RefView::Prev).validate(&view_list));
        assert!(SizeType::Set(0.0).validate(&view_list));
    }

    #[test]
    fn extent_stretch_validate() {
        let view_list = Vec::new();

        assert!(ExtentStretch { pos1: PositionType::Set(0.0), pos2: PositionType::Set(0.0) }.validate(&view_list));
        assert!(!ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), pos2: PositionType::Set(0.0) }.validate(&view_list));
        assert!(!ExtentStretch { pos1: PositionType::Set(0.0), pos2: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }) }.validate(&view_list));
        assert!(!ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), pos2: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }) }.validate(&view_list));
    }

    #[test]
    fn position_type_validate() {
        let view_list = Vec::new();

        assert!(PositionType::Anchor(AnchorPoint { ref_view: RefView::Parent, ref_point: 0.0 }).validate(&view_list));
        assert!(!PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }).validate(&view_list));
        assert!(PositionType::Set(0.0).validate(&view_list));
    }

    #[test]
    fn anchor_point_validate() {
        let mut view_list = Vec::new();
        view_list.push(Box::new(View::new_root()));

        assert!(AnchorPoint {ref_view: RefView::Id(0), ref_point: 0.0}.validate(&view_list));
        assert!(!AnchorPoint {ref_view: RefView::Id(1), ref_point: 0.0}.validate(&view_list));
    }

    #[test]
    fn ref_view_validate() {
        let mut view_list = Vec::new();
        let empty_view_list = Vec::new();
        
        view_list.push(Box::new(View::new_root()));
        view_list.push(Box::new(View::new_root()));

        assert!(RefView::Parent.validate(&view_list));
        assert!(RefView::Prev.validate(&view_list));
        assert!(!RefView::Prev.validate(&empty_view_list));
        assert!(RefView::Id(1).validate(&view_list));
        assert!(!RefView::Id(2).validate(&view_list));
    }
}