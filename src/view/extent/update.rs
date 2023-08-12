use crate::view::{View, ChildValidateError};
use super::Ratio;

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

    /// Tests whether the possible reference views exists, returns an error in case of an invalid reference
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    /// 
    /// # Errors
    /// 
    /// WrongId: If a reference to a sibling by ID is invalid, it is invalid if the ID is larger than the number of children
    /// 
    /// NoPrev: If a reference to the previous sibling is used but this is the first child
    /// 
    /// BothRatio: Both x and y uses a fixed aspect ratio referencing each other
    pub(super) fn validate(&mut self, siblings: &[Box<View>]) -> Result<(), ChildValidateError> {
        // Figure out if any dimension uses ratio and make sure they do not both do that
        let x_ratio = if let ExtentUpdateType::Ratio(_) = self.x.extent_type {
            self.fixed = Dim::Y;
            true
        } else {
            false
        };

        if let ExtentUpdateType::Ratio(_) = self.y.extent_type {
            if x_ratio {
                return Err(ChildValidateError::BothRatio);
            }

            self.fixed = Dim::X;
        };

        // Make sure both x and y are valid
        self.x.validate(siblings)?;
        self.y.validate(siblings)
    }

    /// Retrieves the extent
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// 
    /// siblings: The list of older siblings
    pub(super) fn get(&self, siblings: &[Box<View>], parent_ratio: Ratio) -> (f32, f32, f32, f32) {
        // Get the x and y components
        let (x, y) = match self.fixed {
            // x must be evaluated before y
            Dim::X => {
                let x = self.x.get(Dim::X, siblings, parent_ratio, 0.0);
                let y = self.y.get(Dim::Y, siblings, parent_ratio, x.1);
    
                (x, y)  
            }

            // y must be evaluated before x
            Dim::Y => {
                let y = self.y.get(Dim::Y, siblings, parent_ratio, 0.0);
                let x = self.x.get(Dim::X, siblings, parent_ratio, y.1);
    
                (x, y)  
            }
        };

        (x.0, y.0, x.1, y.1)
    }

    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_insert(&mut self, pos: usize) {
        self.x.update_insert(pos);
        self.y.update_insert(pos);
    }

    /// Updates possible references by ID on deletion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_delete(&mut self, pos: usize) {
        self.x.update_delete(pos);
        self.y.update_delete(pos);
    }

    /// Checks if the ID is being referenced
    /// 
    /// # Parameters
    /// 
    /// id: The ID to check
    fn check_id(&self, id: usize) -> bool {
        self.y.check_id(id) || self.y.check_id(id)
    }

    /// Checks if this view references the previous sibling
    fn check_prev(&self) -> bool {
        self.x.check_prev() || self.y.check_prev()
    }
}

impl ExtentUpdateSingle {
    /// Tests whether the possible reference views exists, returns an error in case of an invalid reference
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    /// 
    /// # Errors
    /// 
    /// WrongId: If a reference to a sibling by ID is invalid, it is invalid if the ID is larger than the number of children
    /// 
    /// NoPrev: If a reference to the previous sibling is used but this is the first child
    fn validate(&self, siblings: &[Box<View>]) -> Result<(), ChildValidateError> {
        // Make sure the extent is valid
        self.extent_type.validate(siblings)
    }

    /// Retrieves the position and size
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// 
    /// siblings: The list of older siblings
    ///
    /// parent_ratio: The aspect ratio of the parent, used if extent type is ratio
    /// 
    /// other_size: The sizze of the other dimension, used if extent type is ratio
    fn get(&self, dim: Dim, siblings: &[Box<View>], parent_ratio: Ratio, other_size: f32) -> (f32, f32) {
        // Get the base position and size
        let (mut pos, mut size) = self.extent_type.get(dim, siblings, parent_ratio, other_size);

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

    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_insert(&mut self, pos: usize) {
        self.extent_type.update_insert(pos);
    }

    /// Updates possible references by ID on deletion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_delete(&mut self, pos: usize) {
        self.extent_type.update_delete(pos);
    }

    /// Checks if the ID is being referenced
    /// 
    /// # Parameters
    /// 
    /// id: The ID to check
    fn check_id(&self, id: usize) -> bool {
        self.extent_type.check_id(id)
    }

    /// Checks if this view references the previous sibling
    fn check_prev(&self) -> bool {
        self.extent_type.check_prev()
    }
}

impl ExtentUpdateType {
    /// Tests whether the possible reference views exists, returns an error in case of an invalid reference
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    /// 
    /// # Errors
    /// 
    /// WrongId: If a reference to a sibling by ID is invalid, it is invalid if the ID is larger than the number of children
    /// 
    /// NoPrev: If a reference to the previous sibling is used but this is the first child
    fn validate(&self, siblings: &[Box<View>]) -> Result<(), ChildValidateError> {
        match self {
            // Make sure stretch mode is valid
            Self::Stretch(stretch) => stretch.validate(siblings),

            // Make sure locate mode is valid
            Self::Locate(locate) => locate.validate(siblings),

            // Make sure ratio mode is valid
            Self::Ratio(ratio) => ratio.validate(siblings),
        }
    }

    /// Retrieves the position and size
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// 
    /// siblings: The list of older siblings
    /// 
    /// parent_ratio: The aspect ratio of the parent, used if extent type is ratio
    /// 
    /// other_size: The sizze of the other dimension, used if extent type is ratio
    fn get(&self, dim: Dim, siblings: &[Box<View>], parent_ratio: Ratio, other_size: f32) -> (f32, f32) {
        match self {
            // Get from the stretch method
            Self::Stretch(stretch) => stretch.get(dim, siblings),

            // Get from the locate method
            Self::Locate(locate) => locate.get(dim, siblings),

            // Get from the ratio method
            Self::Ratio(ratio) => ratio.get(dim, siblings, parent_ratio, other_size),
        }
    }

    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_insert(&mut self, pos: usize) {
        match self {
            // Extent is stretched between two points
            Self::Stretch(stretch) => stretch.update_insert(pos),

            // Extent is defined by a position and size
            Self::Locate(locate) => locate.update_insert(pos),

            // Extent is defined by a position and a ratio to the other dimension size
            Self::Ratio(ratio) => ratio.update_insert(pos),
        }
    }

    /// Updates possible references by ID on deletion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_delete(&mut self, pos: usize) {
        match self {
            // Extent is stretched between two points
            Self::Stretch(stretch) => stretch.update_delete(pos),

            // Extent is defined by a position and size
            Self::Locate(locate) => locate.update_delete(pos),

            // Extent is defined by a position and a ratio to the other dimension size
            Self::Ratio(ratio) => ratio.update_delete(pos),
        }
    }

    /// Checks if the ID is being referenced
    /// 
    /// # Parameters
    /// 
    /// id: The ID to check
    fn check_id(&self, id: usize) -> bool {
        match self {
            // Extent is stretched between two points
            Self::Stretch(stretch) => stretch.check_id(id),

            // Extent is defined by a position and size
            Self::Locate(locate) => locate.check_id(id),

            // Extent is defined by a position and a ratio to the other dimension size
            Self::Ratio(ratio) => ratio.check_id(id),
        }
    }

    /// Checks if this view references the previous sibling
    fn check_prev(&self) -> bool {
        match self {
            // Extent is stretched between two points
            Self::Stretch(stretch) => stretch.check_prev(),

            // Extent is defined by a position and size
            Self::Locate(locate) => locate.check_prev(),

            // Extent is defined by a position and a ratio to the other dimension size
            Self::Ratio(ratio) => ratio.check_prev(),
        }
    }
}

impl ExtentRatio {
    /// Tests whether the possible reference views exists, returns an error in case of an invalid reference
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    /// 
    /// # Errors
    /// 
    /// WrongId: If a reference to a sibling by ID is invalid, it is invalid if the ID is larger than the number of children
    /// 
    /// NoPrev: If a reference to the previous sibling is used but this is the first child
    fn validate(&self, siblings: &[Box<View>]) -> Result<(), ChildValidateError> {
        self.pos.validate(siblings)
    }
    
    /// Retrieves the position and size
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// 
    /// siblings: The list of older siblings
    /// 
    /// parent_ratio: The aspect ratio of the parent
    /// 
    /// other_size: The sizze of the other dimension
    fn get(&self, dim: Dim, siblings: &[Box<View>], parent_ratio: Ratio, other_size: f32) -> (f32, f32) {
        // Get the position and size
        let pos = self.pos.get(dim, siblings);
        let size = other_size / parent_ratio.get();

        (pos, size)
    }

    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_insert(&mut self, pos: usize) {
        self.pos.update_insert(pos);
    }

    /// Updates possible references by ID on deletion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_delete(&mut self, pos: usize) {
        self.pos.update_delete(pos);
    }

    /// Checks if the ID is being referenced
    /// 
    /// # Parameters
    /// 
    /// id: The ID to check
    fn check_id(&self, id: usize) -> bool {
        self.pos.check_id(id)
    }

    /// Checks if this view references the previous sibling
    fn check_prev(&self) -> bool {
        self.pos.check_prev()
    }
}

impl ExtentLocate {
    /// Tests whether the possible reference views exists, returns an error in case of an invalid reference
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    /// 
    /// # Errors
    /// 
    /// WrongId: If a reference to a sibling by ID is invalid, it is invalid if the ID is larger than the number of children
    /// 
    /// NoPrev: If a reference to the previous sibling is used but this is the first child
    fn validate(&self, siblings: &[Box<View>]) -> Result<(), ChildValidateError> {
        // Make sure position and size are valid
        self.pos.validate(siblings)?;
        self.size.validate(siblings)
    }
    
    /// Retrieves the position and size
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// 
    /// siblings: The list of older siblings
    fn get(&self, dim: Dim, siblings: &[Box<View>]) -> (f32, f32) {
        // Get the position and size
        let pos = self.pos.get(dim, siblings);
        let size = self.size.get(dim, siblings);

        (pos, size)
    }

    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_insert(&mut self, pos: usize) {
        self.pos.update_insert(pos);
        self.size.update_delete(pos);
    }

    /// Updates possible references by ID on deletion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_delete(&mut self, pos: usize) {
        self.pos.update_delete(pos);
        self.size.update_delete(pos);
    }

    /// Checks if the ID is being referenced
    /// 
    /// # Parameters
    /// 
    /// id: The ID to check
    fn check_id(&self, id: usize) -> bool {
        self.pos.check_id(id) || self.size.check_id(id)
    }

    /// Checks if this view references the previous sibling
    fn check_prev(&self) -> bool {
        self.pos.check_prev() || self.size.check_prev()
    }
}

impl SizeType {
    /// Tests whether the possible reference views exists, returns an error in case of an invalid reference
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    /// 
    /// # Errors
    /// 
    /// WrongId: If a reference to a sibling by ID is invalid, it is invalid if the ID is larger than the number of children
    /// 
    /// NoPrev: If a reference to the previous sibling is used but this is the first child
    fn validate(&self, siblings: &[Box<View>]) -> Result<(), ChildValidateError> {
        match self {
            // Make sure possible references in the stretch are valid
            Self::Stretch(stretch) => stretch.validate(siblings),

            // Make sure reference is valid
            Self::Relative(ref_view) => ref_view.validate(siblings),

            // Set is always valid
            Self::Set(_) => Ok(()),
        }
    }

    /// Retrieves the size
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// 
    /// siblings: The list of older siblings
    fn get(&self, dim: Dim, siblings: &[Box<View>]) -> f32 {
        match self {
            // Use the size from a stretch
            Self::Stretch(stretch) => stretch.get(dim, siblings).1,

            // Get the size from another view
            Self::Relative(ref_view) => ref_view.get(dim, siblings).1,

            // Use a static size
            Self::Set(size) => *size,
        }
    }

    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_insert(&mut self, pos: usize) {
        match self {
            // The size is relative to another size
            Self::Relative(relative) => relative.update_insert(pos),

            // The size is stretched between two points
            Self::Stretch(stretch) => stretch.update_insert(pos),

            // Set never references anything
            Self::Set(_) => (),
        }
    }

    /// Updates possible references by ID on deletion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_delete(&mut self, pos: usize) {
        match self {
            // The size is relative to another size
            Self::Relative(relative) => relative.update_delete(pos),

            // The size is stretched between two points
            Self::Stretch(stretch) => stretch.update_delete(pos),

            // Set never references anything
            Self::Set(_) => (),
        }
    }

    /// Checks if the ID is being referenced
    /// 
    /// # Parameters
    /// 
    /// id: The ID to check
    fn check_id(&self, id: usize) -> bool {
        match self {
            // The size is relative to another size
            Self::Relative(relative) => relative.check_id(id),

            // The size is stretched between two points
            Self::Stretch(stretch) => stretch.check_id(id),

            // Set never references anything
            Self::Set(_) => false,
        }
    }

    /// Checks if this view references the previous sibling
    fn check_prev(&self) -> bool {
        match self {
            // The size is relative to another size
            Self::Relative(relative) => relative.check_prev(),

            // The size is stretched between two points
            Self::Stretch(stretch) => stretch.check_prev(),

            // Set never references anything
            Self::Set(_) => false,
        }
    }
}

impl ExtentStretch {
    /// Tests whether the possible reference views exists, returns an error in case of an invalid reference
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    /// 
    /// # Errors
    /// 
    /// WrongId: If a reference to a sibling by ID is invalid, it is invalid if the ID is larger than the number of children
    /// 
    /// NoPrev: If a reference to the previous sibling is used but this is the first child
    fn validate(&self, siblings: &[Box<View>]) -> Result<(), ChildValidateError> {
        // Make sure both positions are valid
        self.pos1.validate(siblings)?;
        self.pos2.validate(siblings)
    }

    /// Retrieves the position and size
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// 
    /// siblings: The list of older siblings
    fn get(&self, dim: Dim, siblings: &[Box<View>]) -> (f32, f32) {
        // Get the two positions
        let pos1 = self.pos1.get(dim, siblings);
        let pos2 = self.pos2.get(dim, siblings);

        (pos1, pos2 - pos1)
    }

    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_insert(&mut self, pos: usize) {
        self.pos1.update_insert(pos);
        self.pos2.update_insert(pos);
    }

    /// Updates possible references by ID on deletion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_delete(&mut self, pos: usize) {
        self.pos1.update_delete(pos);
        self.pos2.update_delete(pos);
    }

    /// Checks if the ID is being referenced
    /// 
    /// # Parameters
    /// 
    /// id: The ID to check
    fn check_id(&self, id: usize) -> bool {
        self.pos1.check_id(id) || self.pos2.check_id(id)
    }

    /// Checks if this view references the previous sibling
    fn check_prev(&self) -> bool {
        self.pos1.check_prev() || self.pos2.check_prev()
    }
}

impl PositionType {
    /// Tests whether the possible reference views exists, returns an error in case of an invalid reference
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    /// 
    /// # Errors
    /// 
    /// WrongId: If a reference to a sibling by ID is invalid, it is invalid if the ID is larger than the number of children
    /// 
    /// NoPrev: If a reference to the previous sibling is used but this is the first child
    fn validate(&self, siblings: &[Box<View>]) -> Result<(), ChildValidateError> {
        match self {
            // Make sure the anchor point is valid
            Self::Anchor(anchor) => anchor.validate(siblings),

            // Set is always valid
            Self::Set(_) => Ok(()),
        }
    }

    /// Retrieves the position
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// 
    /// siblings: The list of older siblings
    fn get(&self, dim: Dim, siblings: &[Box<View>]) -> f32 {
        match self {
            // Get from an anchor point
            Self::Anchor(anchor) => anchor.get(dim, siblings),

            // Get a static position
            Self::Set(pos) => *pos,
        }
    }

    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_insert(&mut self, pos: usize) {
        match self {
            // Check the anchor
            Self::Anchor(anchor) => anchor.update_insert(pos),

            // Set is always false
            Self::Set(_) => (),
        }
    }

    /// Updates possible references by ID on deletion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_delete(&mut self, pos: usize) {
        match self {
            // Check the anchor
            Self::Anchor(anchor) => anchor.update_delete(pos),

            // Set is always false
            Self::Set(_) => (),
        }
    }

    /// Checks if the ID is being referenced
    /// 
    /// # Parameters
    /// 
    /// id: The ID to check
    fn check_id(&self, id: usize) -> bool {
        match self {
            // Check the anchor
            Self::Anchor(anchor) => anchor.check_id(id),

            // Set is always false
            Self::Set(_) => false,
        }
    }

    /// Checks if this view references the previous sibling
    fn check_prev(&self) -> bool {
        match self {
            // Check the anchor
            Self::Anchor(anchor) => anchor.check_prev(),

            // Set is always false
            Self::Set(_) => false,
        }
    }
}

impl AnchorPoint {
    /// Tests whether the reference view exists, returns an error in case of an invalid reference
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    /// 
    /// # Errors
    /// 
    /// WrongId: If a reference to a sibling by ID is invalid, it is invalid if the ID is larger than the number of children
    /// 
    /// NoPrev: If a reference to the previous sibling is used but this is the first child
    fn validate(&self, siblings: &[Box<View>]) -> Result<(), ChildValidateError> {
        // Make sure the reference is valid
        self.ref_view.validate(siblings)
    }

    /// Retrieves the anchor point
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// 
    /// siblings: The list of older siblings
    fn get(&self, dim: Dim, siblings: &[Box<View>]) -> f32 {
        // Get the position and size
        let (pos, size) = self.ref_view.get(dim, siblings);

        // Get the correct position
        pos + self.ref_point * size
    }

    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_insert(&mut self, pos: usize) {
        self.ref_view.update_insert(pos);
    }

    /// Updates possible references by ID on deletion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_delete(&mut self, pos: usize) {
        self.ref_view.update_delete(pos);
    }

    /// Checks if the ID is being referenced
    /// 
    /// # Parameters
    /// 
    /// id: The ID to check
    fn check_id(&self, id: usize) -> bool {
        self.ref_view.check_id(id)
    }

    /// Checks if this view references the previous sibling
    fn check_prev(&self) -> bool {
        self.ref_view.check_prev()
    }
}

impl RefView {
    /// Tests whether the reference view exists, returns an error in case of an invalid reference
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    /// 
    /// # Errors
    /// 
    /// WrongId: If a reference to a sibling by ID is invalid, it is invalid if the ID is larger than the number of children
    /// 
    /// NoPrev: If a reference to the previous sibling is used but this is the first child
    fn validate(&self, siblings: &[Box<View>]) -> Result<(), ChildValidateError> {
        match *self {
            // Make sure the index is within the sibling list
            Self::Id(index) => {
                if index >= siblings.len() {
                    Err(ChildValidateError::WrongId(index, siblings.len()))
                } else {
                    Ok(())
                }
            }

            // Make sure the is a sibling if it references the previous
            Self::Prev => {
                if siblings.len() == 0 {
                    Err(ChildValidateError::NoPrev)
                } else {
                    Ok(())
                }
            }
        }
    }

    /// Retrieves the position and size of the view in the given dimension
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// 
    /// siblings: The list of older siblings
    fn get(&self, dim: Dim, siblings: &[Box<View>]) -> (f32, f32) {
        match *self {
            // Get size from previous sibling
            Self::Prev => dim.get_from_view(siblings.last().unwrap()),

            // Get size from id
            Self::Id(n) => dim.get_from_view(siblings.get(n).unwrap()),
        }
    }

    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_insert(&mut self, pos: usize) {
        if let Self::Id(id) = self {
            if *id >= pos {
                *id += 1;
            }
        }
    }

    /// Updates possible references by ID on deletion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted
    fn update_delete(&mut self, pos: usize) {
        if let Self::Id(id) = self {
            if *id > pos {
                *id -= 1;
            }
        }
    }

    /// Checks if the ID is being referenced
    /// 
    /// # Parameters
    /// 
    /// id: The ID to check
    fn check_id(&self, id: usize) -> bool {
        if let Self::Id(use_id) = *self {
            if id == use_id {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Checks if this view references the previous sibling
    fn check_prev(&self) -> bool {
        if let Self::Prev = *self {
            true
        } else {
            false
        }
    }
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
    use super::*;

    #[test]
    fn extent_update_validate() {
        let view_list = Vec::new();
        let single_extent_success = ExtentUpdateSingle { extent_type: ExtentUpdateType::Stretch(ExtentStretch { pos1: PositionType::Set(0.0), pos2: PositionType::Set(0.0) }), scale_rel: 0.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 };
        let single_extent_fail = ExtentUpdateSingle { extent_type: ExtentUpdateType::Locate(ExtentLocate {pos: PositionType::Set(0.0), size: SizeType::Relative(RefView::Prev)}), scale_rel: 0.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 };

        assert!(ExtentUpdate { x: single_extent_success, y: single_extent_success }.validate(&view_list).is_ok());
        assert!(ExtentUpdate { x: single_extent_fail, y: single_extent_success }.validate(&view_list).is_err());
        assert!(ExtentUpdate { x: single_extent_success, y: single_extent_fail }.validate(&view_list).is_err());
        assert!(ExtentUpdate { x: single_extent_fail, y: single_extent_fail }.validate(&view_list).is_err());
    }

    #[test]
    fn extent_update_single_validate() {
        let view_list = Vec::new();

        assert!(ExtentUpdateSingle { extent_type: ExtentUpdateType::Stretch(ExtentStretch { pos1: PositionType::Set(0.0), pos2: PositionType::Set(0.0) }), scale_rel: 0.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 }.validate(&view_list).is_ok());
        assert!(ExtentUpdateSingle { extent_type: ExtentUpdateType::Locate(ExtentLocate {pos: PositionType::Set(0.0), size: SizeType::Relative(RefView::Prev)}), scale_rel: 0.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 }.validate(&view_list).is_err());
    }

    #[test]
    fn extent_update_type_validate() {
        let view_list = Vec::new();

        assert!(ExtentUpdateType::Stretch(ExtentStretch { pos1: PositionType::Set(0.0), pos2: PositionType::Set(0.0) }).validate(&view_list).is_ok());
        assert!(ExtentUpdateType::Stretch(ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), pos2: PositionType::Set(0.0) }).validate(&view_list).is_err());
        assert!(ExtentUpdateType::Locate(ExtentLocate {pos: PositionType::Set(0.0), size: SizeType::Set(0.0)}).validate(&view_list).is_ok());
        assert!(ExtentUpdateType::Locate(ExtentLocate {pos: PositionType::Set(0.0), size: SizeType::Relative(RefView::Prev)}).validate(&view_list).is_err());
    }

    #[test]
    fn extent_locate_validate() {
        let view_list = Vec::new();

        assert!(ExtentLocate {pos: PositionType::Set(0.0), size: SizeType::Set(0.0)}.validate(&view_list).is_ok());
        assert!(ExtentLocate {pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), size: SizeType::Set(0.0)}.validate(&view_list).is_err());
        assert!(ExtentLocate {pos: PositionType::Set(0.0), size: SizeType::Relative(RefView::Prev)}.validate(&view_list).is_err());
        assert!(ExtentLocate {pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), size: SizeType::Relative(RefView::Prev)}.validate(&view_list).is_err());
    }

    #[test]
    fn size_type_validate() {
        let view_list = Vec::new();

        assert!(SizeType::Stretch(ExtentStretch { pos1: PositionType::Set(0.0), pos2: PositionType::Set(0.0) }).validate(&view_list).is_ok());
        assert!(SizeType::Stretch(ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), pos2: PositionType::Set(0.0) }).validate(&view_list).is_err());
        //assert!(SizeType::Relative(RefView::Parent).validate(&view_list).is_ok());
        assert!(SizeType::Relative(RefView::Prev).validate(&view_list).is_err());
        assert!(SizeType::Set(0.0).validate(&view_list).is_ok());
    }

    #[test]
    fn extent_stretch_validate() {
        let view_list = Vec::new();

        assert!(ExtentStretch { pos1: PositionType::Set(0.0), pos2: PositionType::Set(0.0) }.validate(&view_list).is_ok());
        assert!(ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), pos2: PositionType::Set(0.0) }.validate(&view_list).is_err());
        assert!(ExtentStretch { pos1: PositionType::Set(0.0), pos2: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }) }.validate(&view_list).is_err());
        assert!(ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), pos2: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }) }.validate(&view_list).is_err());
    }

    #[test]
    fn position_type_validate() {
        let view_list = Vec::new();

        //assert!(PositionType::Anchor(AnchorPoint { ref_view: RefView::Parent, ref_point: 0.0 }).validate(&view_list).is_ok());
        assert!(PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }).validate(&view_list).is_err());
        assert!(PositionType::Set(0.0).validate(&view_list).is_ok());
    }

    #[test]
    fn anchor_point_validate() {
        let mut view_list = Vec::new();
        view_list.push(View::new_root());

        assert!(AnchorPoint {ref_view: RefView::Id(0), ref_point: 0.0}.validate(&view_list).is_ok());
        assert!(AnchorPoint {ref_view: RefView::Id(1), ref_point: 0.0}.validate(&view_list).is_err());
    }

    #[test]
    fn ref_view_validate() {
        let mut view_list = Vec::new();
        let empty_view_list = Vec::new();
        
        view_list.push(View::new_root());
        view_list.push(View::new_root());

        //assert!(RefView::Parent.validate(&view_list).is_ok());
        assert!(RefView::Prev.validate(&view_list).is_ok());
        assert!(RefView::Prev.validate(&empty_view_list).is_err());
        assert!(RefView::Id(1).validate(&view_list).is_ok());
        assert!(RefView::Id(2).validate(&view_list).is_err());
    }
}