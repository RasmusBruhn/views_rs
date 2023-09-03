use super::{View, ChildValidateError};
use super::{ExtentUpdate, ExtentUpdateType, ExtentUpdateSingle, ExtentStretch, ExtentLocate, ExtentRatio, Dim, SizeType, PositionType, AnchorPoint, RefView};

impl ExtentUpdate {
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
    pub(crate) fn validate(&mut self, siblings: &[Box<View>]) -> Result<(), ChildValidateError> {
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
