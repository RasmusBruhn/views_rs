use super::{ExtentUpdate, ExtentUpdateType, ExtentUpdateSingle, ExtentStretch, ExtentLocate, ExtentRatio, SizeType, PositionType, AnchorPoint, RefView};

impl ExtentUpdate {
    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted into
    pub(crate) fn update_insert(&mut self, pos: usize) {
        self.x.update_insert(pos);
        self.y.update_insert(pos);
    }

    /// Updates possible references by ID on deletion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was deleted from
    pub(crate) fn update_delete(&mut self, pos: usize) {
        self.x.update_delete(pos);
        self.y.update_delete(pos);
    }
}

impl ExtentUpdateSingle {
    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted into
    pub(crate) fn update_insert(&mut self, pos: usize) {
        self.extent_type.update_insert(pos);
    }

    /// Updates possible references by ID on deletion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was deleted from
    pub(crate) fn update_delete(&mut self, pos: usize) {
        self.extent_type.update_delete(pos);
    }
}

impl ExtentUpdateType {
    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted into
    pub(crate) fn update_insert(&mut self, pos: usize) {
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
    /// pos: The position that the sibling was deleted from
    pub(crate) fn update_delete(&mut self, pos: usize) {
        match self {
            // Extent is stretched between two points
            Self::Stretch(stretch) => stretch.update_delete(pos),

            // Extent is defined by a position and size
            Self::Locate(locate) => locate.update_delete(pos),

            // Extent is defined by a position and a ratio to the other dimension size
            Self::Ratio(ratio) => ratio.update_delete(pos),
        }
    }
}

impl ExtentRatio {
    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted into
    pub(crate) fn update_insert(&mut self, pos: usize) {
        self.pos.update_insert(pos);
    }

    /// Updates possible references by ID on deletion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was deleted from
    pub(crate) fn update_delete(&mut self, pos: usize) {
        self.pos.update_delete(pos);
    }
}

impl ExtentLocate {
    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted into
    pub(crate) fn update_insert(&mut self, pos: usize) {
        self.pos.update_insert(pos);
        self.size.update_delete(pos);
    }

    /// Updates possible references by ID on deletion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was deleted from
    pub(crate) fn update_delete(&mut self, pos: usize) {
        self.pos.update_delete(pos);
        self.size.update_delete(pos);
    }
}

impl SizeType {
    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted into
    pub(crate) fn update_insert(&mut self, pos: usize) {
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
    /// pos: The position that the sibling was deleted from
    pub(crate) fn update_delete(&mut self, pos: usize) {
        match self {
            // The size is relative to another size
            Self::Relative(relative) => relative.update_delete(pos),

            // The size is stretched between two points
            Self::Stretch(stretch) => stretch.update_delete(pos),

            // Set never references anything
            Self::Set(_) => (),
        }
    }
}

impl ExtentStretch {
    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted into
    pub(crate) fn update_insert(&mut self, pos: usize) {
        self.pos1.update_insert(pos);
        self.pos2.update_insert(pos);
    }

    /// Updates possible references by ID on deletion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was deleted from
    pub(crate) fn update_delete(&mut self, pos: usize) {
        self.pos1.update_delete(pos);
        self.pos2.update_delete(pos);
    }
}

impl PositionType {
    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted into
    pub(crate) fn update_insert(&mut self, pos: usize) {
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
    /// pos: The position that the sibling was deleted from
    pub(crate) fn update_delete(&mut self, pos: usize) {
        match self {
            // Check the anchor
            Self::Anchor(anchor) => anchor.update_delete(pos),

            // Set is always false
            Self::Set(_) => (),
        }
    }
}

impl AnchorPoint {
    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted into
    pub(crate) fn update_insert(&mut self, pos: usize) {
        self.ref_view.update_insert(pos);
    }

    /// Updates possible references by ID on deletion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was deleted from
    pub(crate) fn update_delete(&mut self, pos: usize) {
        self.ref_view.update_delete(pos);
    }
}

impl RefView {
    /// Updates possible references by ID on insertion of a sibling before this one
    /// 
    /// # Parameters
    /// 
    /// pos: The position that the sibling was inserted into
    pub(crate) fn update_insert(&mut self, pos: usize) {
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
    /// pos: The position that the sibling was deleted from
    pub(crate) fn update_delete(&mut self, pos: usize) {
        if let Self::Id(id) = self {
            if *id > pos {
                *id -= 1;
            }
        }
    }
}
