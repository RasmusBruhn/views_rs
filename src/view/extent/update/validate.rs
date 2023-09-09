use super::{View, ChildValidateError};
use super::{ExtentUpdate, ExtentUpdateType, ExtentUpdateSingle, ExtentStretch, ExtentLocate, ExtentRatio, SizeType, PositionType, AnchorPoint, RefView};

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
    pub(crate) fn validate(&self, siblings: &[Box<View>]) -> Result<(), ChildValidateError> {
        // Make sure both dimensions are not using ratio mode
        if let ExtentUpdateType::Ratio(_) = self.x.extent_type {
            if let ExtentUpdateType::Ratio(_) = self.y.extent_type {
                return Err(ChildValidateError::BothRatio);
            }
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
        self.x.check_id(id) || self.y.check_id(id)
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

            // Make sure there is a sibling if it references the previous
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

#[cfg(test)]
mod tests {
    use crate::view::extent::{Extent, Ratio};
    use super::*;

    fn gen_view(x: f32, y: f32, w: f32, h: f32, sibling_id: usize) -> View {
        let extent_single = ExtentUpdateSingle { scale_rel: 1.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0, extent_type: ExtentUpdateType::Locate(ExtentLocate { pos: PositionType::Set(0.0), size: SizeType::Set(1.0) }) };
        let extent_info = ExtentUpdate { x: extent_single, y: extent_single };
        let mut extent = Extent::new(extent_info);
        extent.x = x;
        extent.y = y;
        extent.w = w;
        extent.h = h;
        extent.ratio = Ratio::new(w, h);
        View { children: Vec::new(), extent: extent }
    }

    mod validate {
        use super::*;

        #[test]
        fn ref_view() {
            let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
            let sibling2 = gen_view(5.0, 6.0, 7.0, 8.0, 1);
            let siblings = vec![Box::new(sibling1), Box::new(sibling2)];

            let ref_view_id = RefView::Id(1);
            assert_eq!(Ok(()), ref_view_id.validate(&siblings));
            assert_eq!(Err(ChildValidateError::WrongId(1, 0)), ref_view_id.validate(&siblings[..0]));

            let ref_view_prev = RefView::Prev;
            assert_eq!(Ok(()), ref_view_prev.validate(&siblings));
            assert_eq!(Err(ChildValidateError::NoPrev), ref_view_prev.validate(&siblings[..0]));
        }

        #[test]
        fn anchor_point() {
            let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
            let siblings = vec![Box::new(sibling1)];

            let anchor_point = AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 };
            assert_eq!(Ok(()), anchor_point.validate(&siblings));
            assert_eq!(Err(ChildValidateError::NoPrev), anchor_point.validate(&siblings[..0]));
        }

        #[test]
        fn position_type() {
            let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
            let siblings = vec![Box::new(sibling1)];

            let position_type_set = PositionType::Set(0.0);
            assert_eq!(Ok(()), position_type_set.validate(&siblings));

            let position_type_anchor = PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 });
            assert_eq!(Ok(()), position_type_anchor.validate(&siblings));
            assert_eq!(Err(ChildValidateError::NoPrev), position_type_anchor.validate(&siblings[..0]));
        }

        #[test]
        fn extent_stretch() {
            let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
            let siblings = vec![Box::new(sibling1)];

            let extent_stretch_success = ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), pos2: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }) };
            assert_eq!(Ok(()), extent_stretch_success.validate(&siblings));

            let extent_stretch_fail1 = ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), pos2: PositionType::Set(0.0) };
            assert_eq!(Err(ChildValidateError::NoPrev), extent_stretch_fail1.validate(&siblings[..0]));

            let extent_stretch_fail2 = ExtentStretch { pos1: PositionType::Set(0.0), pos2: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }) };
            assert_eq!(Err(ChildValidateError::NoPrev), extent_stretch_fail2.validate(&siblings[..0]));
        }

        #[test]
        fn size_type() {
            let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
            let siblings = vec![Box::new(sibling1)];

            let size_type_stretch = SizeType::Stretch(ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), pos2: PositionType::Set(0.0) });
            assert_eq!(Ok(()), size_type_stretch.validate(&siblings));
            assert_eq!(Err(ChildValidateError::NoPrev), size_type_stretch.validate(&siblings[..0]));

            let size_type_relative = SizeType::Relative(RefView::Prev);
            assert_eq!(Ok(()), size_type_relative.validate(&siblings));
            assert_eq!(Err(ChildValidateError::NoPrev), size_type_relative.validate(&siblings[..0]));

            let size_type_set = SizeType::Set(0.0);
            assert_eq!(Ok(()), size_type_set.validate(&siblings[..0]));
        }

        #[test]
        fn extent_locate() {
            let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
            let siblings = vec![Box::new(sibling1)];

            let extent_locate_success = ExtentLocate { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), size: SizeType::Relative(RefView::Prev) };
            assert_eq!(Ok(()), extent_locate_success.validate(&siblings));

            let extent_locate_failpos = ExtentLocate { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), size: SizeType::Set(0.0) };
            assert_eq!(Err(ChildValidateError::NoPrev), extent_locate_failpos.validate(&siblings[..0]));

            let extent_locate_failsize = ExtentLocate { pos: PositionType::Set(0.0), size: SizeType::Relative(RefView::Prev) };
            assert_eq!(Err(ChildValidateError::NoPrev), extent_locate_failsize.validate(&siblings[..0]));
        }

        #[test]
        fn extent_ratio() {
            let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
            let siblings = vec![Box::new(sibling1)];

            let extent_ratio = ExtentRatio { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }) };
            assert_eq!(Ok(()), extent_ratio.validate(&siblings));
            assert_eq!(Err(ChildValidateError::NoPrev), extent_ratio.validate(&siblings[..0]));
        }

        #[test]
        fn extent_update_type() {
            let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
            let siblings = vec![Box::new(sibling1)];

            let extent_update_type_stretch = ExtentUpdateType::Stretch(ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), pos2: PositionType::Set(0.0) });
            assert_eq!(Ok(()), extent_update_type_stretch.validate(&siblings));
            assert_eq!(Err(ChildValidateError::NoPrev), extent_update_type_stretch.validate(&siblings[..0]));

            let extent_update_type_locate = ExtentUpdateType::Locate(ExtentLocate { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), size: SizeType::Set(0.0) });
            assert_eq!(Ok(()), extent_update_type_locate.validate(&siblings));
            assert_eq!(Err(ChildValidateError::NoPrev), extent_update_type_locate.validate(&siblings[..0]));

            let extent_update_type_ratio = ExtentUpdateType::Ratio(ExtentRatio { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }) });
            assert_eq!(Ok(()), extent_update_type_ratio.validate(&siblings));
            assert_eq!(Err(ChildValidateError::NoPrev), extent_update_type_ratio.validate(&siblings[..0]));
        }

        #[test]
        fn extent_update_single() {
            let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
            let siblings = vec![Box::new(sibling1)];

            let extent_update_single = ExtentUpdateSingle { extent_type: ExtentUpdateType::Ratio(ExtentRatio { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }) }), scale_rel: 0.0, scale_abs: 0.0, offset_abs: 0.0, offset_rel: 0.0 };
            assert_eq!(Ok(()), extent_update_single.validate(&siblings));
            assert_eq!(Err(ChildValidateError::NoPrev), extent_update_single.validate(&siblings[..0]));
        }

        #[test]
        fn extent_update() {
            let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
            let siblings = vec![Box::new(sibling1)];

            let ratio = ExtentUpdateType::Ratio(ExtentRatio { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }) });
            let locate = ExtentUpdateType::Locate(ExtentLocate { pos: PositionType::Set(0.0), size: SizeType::Set(0.0) });
            let extent_update_success = ExtentUpdate { x: ExtentUpdateSingle { extent_type: ratio, scale_rel: 0.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 }, y: ExtentUpdateSingle { extent_type: locate, scale_rel: 0.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 } };
            assert_eq!(Ok(()), extent_update_success.validate(&siblings));

            let extent_update_failx = ExtentUpdate { x: ExtentUpdateSingle { extent_type: ratio, scale_rel: 0.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 }, y: ExtentUpdateSingle { extent_type: locate, scale_rel: 0.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 } };
            assert_eq!(Err(ChildValidateError::NoPrev), extent_update_failx.validate(&siblings[..0]));

            let extent_update_faily = ExtentUpdate { x: ExtentUpdateSingle { extent_type: locate, scale_rel: 0.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 }, y: ExtentUpdateSingle { extent_type: ratio, scale_rel: 0.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 } };
            assert_eq!(Err(ChildValidateError::NoPrev), extent_update_faily.validate(&siblings[..0]));

            let extent_update_failratio = ExtentUpdate { x: ExtentUpdateSingle { extent_type: ratio, scale_rel: 0.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 }, y: ExtentUpdateSingle { extent_type: ratio, scale_rel: 0.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 } };
            assert_eq!(Err(ChildValidateError::BothRatio), extent_update_failratio.validate(&siblings));
        }
    }

    mod check_id {
        use super::*;

        #[test]
        fn ref_view() {
            let ref_view_id = RefView::Id(1);
            assert!(!ref_view_id.check_id(0));
            assert!(ref_view_id.check_id(1));

            let ref_view_prev = RefView::Prev;
            assert!(!ref_view_prev.check_id(1));
        }

        #[test]
        fn anchor_point() {
            let anchor_point = AnchorPoint { ref_view: RefView::Id(1), ref_point: 0.0 };
            assert!(!anchor_point.check_id(0));
            assert!(anchor_point.check_id(1));
        }

        #[test]
        fn position_type() {
            let position_type_anchor = PositionType::Anchor(AnchorPoint { ref_view: RefView::Id(1), ref_point: 0.0 });
            assert!(!position_type_anchor.check_id(0));
            assert!(position_type_anchor.check_id(1));

            let position_type_set = PositionType::Set(0.0);
            assert!(!position_type_set.check_id(1));
        }

        #[test]
        fn extent_stretch() {
            let extent_stretch_1 = ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Id(1), ref_point: 0.0 }), pos2: PositionType::Set(0.0) };
            assert!(!extent_stretch_1.check_id(0));
            assert!(extent_stretch_1.check_id(1));

            let extent_stretch_2 = ExtentStretch { pos1: PositionType::Set(0.0), pos2: PositionType::Anchor(AnchorPoint { ref_view: RefView::Id(1), ref_point: 0.0 }) };
            assert!(!extent_stretch_2.check_id(0));
            assert!(extent_stretch_2.check_id(1));
        }

        #[test]
        fn size_type() {
            let size_type_stretch = SizeType::Stretch(ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Id(1), ref_point: 0.0 }), pos2: PositionType::Set(0.0) });
            assert!(!size_type_stretch.check_id(0));
            assert!(size_type_stretch.check_id(1));

            let size_type_relative = SizeType::Relative(RefView::Id(1));
            assert!(!size_type_relative.check_id(0));
            assert!(size_type_relative.check_id(1));

            let size_type_set = SizeType::Set(0.0);
            assert!(!size_type_set.check_id(1));
        }

        #[test]
        fn extent_locate() {
            let extent_locate_pos = ExtentLocate { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Id(1), ref_point: 0.0 }), size: SizeType::Set(0.0) };
            assert!(!extent_locate_pos.check_id(0));
            assert!(extent_locate_pos.check_id(1));

            let extent_locate_size = ExtentLocate { pos: PositionType::Set(0.0), size: SizeType::Relative(RefView::Id(1)) };
            assert!(!extent_locate_size.check_id(0));
            assert!(extent_locate_size.check_id(1));
        }

        #[test]
        fn extent_ratio() {
            let extent_ratio = ExtentRatio { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Id(1), ref_point: 0.0 }) };
            assert!(!extent_ratio.check_id(0));
            assert!(extent_ratio.check_id(1));
        }

        #[test]
        fn extent_update_type() {
            let extent_update_type_stretch = ExtentUpdateType::Stretch(ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Id(1), ref_point: 0.0 }), pos2: PositionType::Set(0.0) });
            assert!(!extent_update_type_stretch.check_id(0));
            assert!(extent_update_type_stretch.check_id(1));

            let extent_update_type_locate = ExtentUpdateType::Locate(ExtentLocate { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Id(1), ref_point: 0.0 }), size: SizeType::Set(0.0) });
            assert!(!extent_update_type_locate.check_id(0));
            assert!(extent_update_type_locate.check_id(1));

            let extent_update_type_ratio = ExtentUpdateType::Ratio(ExtentRatio { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Id(1), ref_point: 0.0 }) });
            assert!(!extent_update_type_ratio.check_id(0));
            assert!(extent_update_type_ratio.check_id(1));
        }

        #[test]
        fn extent_update_single() {
            let extent_update_single = ExtentUpdateSingle { extent_type: ExtentUpdateType::Ratio(ExtentRatio { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Id(1), ref_point: 0.0 }) }), scale_abs: 0.0, scale_rel: 0.0, offset_abs: 0.0, offset_rel: 0.0 };
            assert!(!extent_update_single.check_id(0));
            assert!(extent_update_single.check_id(1));
        }

        #[test]
        fn extent_update() {
            let extent_update_single_id = ExtentUpdateSingle { extent_type: ExtentUpdateType::Ratio(ExtentRatio { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Id(1), ref_point: 0.0 }) }), scale_abs: 0.0, scale_rel: 0.0, offset_abs: 0.0, offset_rel: 0.0 };
            let extent_update_single_set = ExtentUpdateSingle { extent_type: ExtentUpdateType::Ratio(ExtentRatio { pos: PositionType::Set(0.0) }), scale_abs: 0.0, scale_rel: 0.0, offset_abs: 0.0, offset_rel: 0.0 };

            let extent_update_x = ExtentUpdate { x: extent_update_single_id, y: extent_update_single_set };
            assert!(!extent_update_x.check_id(0));
            assert!(extent_update_x.check_id(1));

            let extent_update_y = ExtentUpdate { x: extent_update_single_set, y: extent_update_single_id };
            assert!(!extent_update_y.check_id(0));
            assert!(extent_update_y.check_id(1));
        }
    }

    mod check_prev {
        use super::*;

        #[test]
        fn ref_view() {
            let ref_view_id = RefView::Id(1);
            assert!(!ref_view_id.check_prev());

            let ref_view_prev = RefView::Prev;
            assert!(ref_view_prev.check_prev());
        }

        #[test]
        fn anchor_point() {
            let anchor_point_true = AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 };
            assert!(anchor_point_true.check_prev());

            let anchor_point_false = AnchorPoint { ref_view: RefView::Id(0), ref_point: 0.0 };
            assert!(!anchor_point_false.check_prev());
        }

        #[test]
        fn position_type() {
            let position_type_anchor_true = PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 });
            assert!(position_type_anchor_true.check_prev());

            let position_type_anchor_false = PositionType::Anchor(AnchorPoint { ref_view: RefView::Id(0), ref_point: 0.0 });
            assert!(!position_type_anchor_false.check_prev());

            let position_type_set = PositionType::Set(0.0);
            assert!(!position_type_set.check_prev());
        }

        #[test]
        fn extent_stretch() {
            let extent_stretch_1 = ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), pos2: PositionType::Set(0.0) };
            assert!(extent_stretch_1.check_prev());

            let extent_stretch_2 = ExtentStretch { pos1: PositionType::Set(0.0), pos2: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }) };
            assert!(extent_stretch_2.check_prev());

            let extent_stretch_none = ExtentStretch { pos1: PositionType::Set(0.0), pos2: PositionType::Set(0.0) };
            assert!(!extent_stretch_none.check_prev());
        }

        #[test]
        fn size_type() {
            let size_type_stretch_true = SizeType::Stretch(ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), pos2: PositionType::Set(0.0) });
            assert!(size_type_stretch_true.check_prev());

            let size_type_stretch_false=  SizeType::Stretch(ExtentStretch { pos1: PositionType::Set(0.0), pos2: PositionType::Set(0.0) });
            assert!(!size_type_stretch_false.check_prev());

            let size_type_relative_true = SizeType::Relative(RefView::Prev);
            assert!(size_type_relative_true.check_prev());

            let size_type_relative_false = SizeType::Relative(RefView::Id(0));
            assert!(!size_type_relative_false.check_prev());

            let size_type_set = SizeType::Set(0.0);
            assert!(!size_type_set.check_prev());
        }

        #[test]
        fn extent_locate() {
            let extent_locate_pos_true = ExtentLocate { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), size: SizeType::Set(0.0) };
            assert!(extent_locate_pos_true.check_prev());

            let extent_locate_size_true = ExtentLocate { pos: PositionType::Set(0.0), size: SizeType::Relative(RefView::Prev) };
            assert!(extent_locate_size_true.check_prev());

            let extent_locate_none = ExtentLocate { pos: PositionType::Set(0.0), size: SizeType::Set(0.0) };
            assert!(!extent_locate_none.check_prev());
        }

        #[test]
        fn extent_ratio() {
            let extent_ratio_true = ExtentRatio { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }) };
            assert!(extent_ratio_true.check_prev());

            let extent_ratio_false = ExtentRatio { pos: PositionType::Set(0.0) };
            assert!(!extent_ratio_false.check_prev());
        }

        #[test]
        fn extent_update_type() {
            let extent_update_type_stretch_true = ExtentUpdateType::Stretch(ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), pos2: PositionType::Set(0.0) });
            assert!(extent_update_type_stretch_true.check_prev());

            let extent_update_type_stretch_false = ExtentUpdateType::Stretch(ExtentStretch { pos1: PositionType::Set(0.0), pos2: PositionType::Set(0.0) });
            assert!(!extent_update_type_stretch_false.check_prev());

            let extent_update_type_locate_true = ExtentUpdateType::Locate(ExtentLocate { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }), size: SizeType::Set(0.0) });
            assert!(extent_update_type_locate_true.check_prev());

            let extent_update_type_locate_false = ExtentUpdateType::Locate(ExtentLocate { pos: PositionType::Set(0.0), size: SizeType::Set(0.0) });
            assert!(!extent_update_type_locate_false.check_prev());

            let extent_update_type_ratio_true = ExtentUpdateType::Ratio(ExtentRatio { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }) });
            assert!(extent_update_type_ratio_true.check_prev());

            let extent_update_type_ratio_false = ExtentUpdateType::Ratio(ExtentRatio { pos: PositionType::Set(0.0) });
            assert!(!extent_update_type_ratio_false.check_prev());
        }

        #[test]
        fn extent_update_single() {
            let extent_update_single_true = ExtentUpdateSingle { extent_type: ExtentUpdateType::Ratio(ExtentRatio { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }) }), scale_abs: 0.0, scale_rel: 0.0, offset_abs: 0.0, offset_rel: 0.0 };
            assert!(extent_update_single_true.check_prev());

            let extent_update_single_false = ExtentUpdateSingle { extent_type: ExtentUpdateType::Ratio(ExtentRatio { pos: PositionType::Set(0.0) }), scale_abs: 0.0, scale_rel: 0.0, offset_abs: 0.0, offset_rel: 0.0 };
            assert!(!extent_update_single_false.check_prev());
        }

        #[test]
        fn extent_update() {
            let extent_update_single_prev = ExtentUpdateSingle { extent_type: ExtentUpdateType::Ratio(ExtentRatio { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 }) }), scale_abs: 0.0, scale_rel: 0.0, offset_abs: 0.0, offset_rel: 0.0 };
            let extent_update_single_set = ExtentUpdateSingle { extent_type: ExtentUpdateType::Ratio(ExtentRatio { pos: PositionType::Set(0.0) }), scale_abs: 0.0, scale_rel: 0.0, offset_abs: 0.0, offset_rel: 0.0 };

            let extent_update_x = ExtentUpdate { x: extent_update_single_prev, y: extent_update_single_set };
            assert!(extent_update_x.check_prev());

            let extent_update_y = ExtentUpdate { x: extent_update_single_set, y: extent_update_single_prev };
            assert!(extent_update_y.check_prev());

            let extent_update_none = ExtentUpdate { x: extent_update_single_set, y: extent_update_single_set };
            assert!(!extent_update_none.check_prev());
        }
    }
}