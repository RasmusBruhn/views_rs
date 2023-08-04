use crate::view::View;

#[derive(Clone, Copy, Debug)]
pub struct ExtentUpdate {
    pub x: ExtentUpdateSingle,
    pub y: ExtentUpdateSingle,  
}

#[derive(Clone, Copy, Debug)]
pub struct ExtentUpdateSingle {
    pub extent_type: ExtentUpdateType,
    pub scale_rel: f32,
    pub scale_abs: f32,
    pub offset_rel: f32,
    pub offset_abs: f32,    
}

#[derive(Clone, Copy, Debug)]
pub enum ExtentUpdateType {
    Stretch(ExtentStretch),
    Locate(ExtentLocate),
}

#[derive(Clone, Copy, Debug)]
pub struct ExtentLocate {
    pub pos: PositionType,
    pub size: SizeType,
}

#[derive(Clone, Copy, Debug)]
pub enum SizeType {
    Stretch(ExtentStretch),
    Relative(RefView),
    Set(f32),
}

#[derive(Clone, Copy, Debug)]
pub struct ExtentStretch {
    pub pos1: PositionType,
    pub pos2: PositionType,
}

#[derive(Clone, Copy, Debug)]
pub enum PositionType {
    Anchor(AnchorPoint),
    Set(f32),
}

#[derive(Clone, Copy, Debug)]
pub struct AnchorPoint {
    pub ref_view: RefView,
    pub ref_point: f32,
}

#[derive(Clone, Copy, Debug)]
pub enum RefView {
    Parent,
    Prev,
    Id(usize),
}

impl ExtentUpdate {
    /// Tests wether the possible reference views exists
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    pub(crate) fn validate(&self, siblings: &[Box<View>]) -> bool {
        // Make sure both x and y are valid
        self.x.validate(siblings) && self.y.validate(siblings)
    }
}

impl ExtentUpdateSingle {
    /// Tests wether the possible reference views exists
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    pub(crate) fn validate(&self, siblings: &[Box<View>]) -> bool {
        // Make sure the extent is valid
        self.extent_type.validate(siblings)
    }
}

impl ExtentUpdateType {
    /// Tests wether the possible reference views exists
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    pub(crate) fn validate(&self, siblings: &[Box<View>]) -> bool {
        match *self {
            // Make sure stretch mode is valid
            ExtentUpdateType::Stretch(stretch) => stretch.validate(siblings),

            // Make sure locate mode is valid
            ExtentUpdateType::Locate(locate) => locate.validate(siblings),
        }
    }
}

impl ExtentLocate {
    /// Tests wether the possible reference views exists
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    pub(crate) fn validate(&self, siblings: &[Box<View>]) -> bool {
        // Make sure position and size are valid
        self.pos.validate(siblings) && self.size.validate(siblings)
    }
}

impl SizeType {
    /// Tests wether the possible reference views exists
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    pub(crate) fn validate(&self, siblings: &[Box<View>]) -> bool {
        match *self {
            // Make sure possible references in the stretch are valid
            SizeType::Stretch(stretch) => stretch.validate(siblings),

            // Make sure reference is valid
            SizeType::Relative(ref_view) => ref_view.validate(siblings),

            // Set is always valid
            SizeType::Set(_) => true,
        }
    }
}

impl ExtentStretch {
    /// Tests wether the possible reference views exists
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    pub(crate) fn validate(&self, siblings: &[Box<View>]) -> bool {
        // Make sure both positions are valid
        self.pos1.validate(siblings) && self.pos2.validate(siblings)
    }
}

impl PositionType {
    /// Tests wether the possible reference views exists
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    pub(crate) fn validate(&self, siblings: &[Box<View>]) -> bool {
        match *self {
            // Make sure the anchor point is valid
            PositionType::Anchor(anchor) => anchor.validate(siblings),

            // Set is always valid
            PositionType::Set(_) => true,
        }
    }
}

impl AnchorPoint {
    /// Tests wether the reference view exists
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    pub(crate) fn validate(&self, siblings: &[Box<View>]) -> bool {
        // Make sure the reference is valid
        self.ref_view.validate(siblings)
    }
}

impl RefView {
    /// Tests wether the reference view exists
    /// 
    /// # Parameters
    /// 
    /// siblings: A slice of all the previous siblings of this view
    pub(crate) fn validate(&self, siblings: &[Box<View>]) -> bool {
        match *self {
            // Make sure the index is within the sibling list
            RefView::Id(index) => siblings.len() > index,

            // Make sure the is a sibling if it references the previous
            RefView::Prev => siblings.len() > 0,

            // There is always a parent
            RefView::Parent => true,
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
        view_list.push(Box::new(View::new()));

        assert!(AnchorPoint {ref_view: RefView::Id(0), ref_point: 0.0}.validate(&view_list));
        assert!(!AnchorPoint {ref_view: RefView::Id(1), ref_point: 0.0}.validate(&view_list));
    }

    #[test]
    fn ref_view_validate() {
        let mut view_list = Vec::new();
        let empty_view_list = Vec::new();
        
        view_list.push(Box::new(View::new()));
        view_list.push(Box::new(View::new()));

        assert!(RefView::Parent.validate(&view_list));
        assert!(RefView::Prev.validate(&view_list));
        assert!(!RefView::Prev.validate(&empty_view_list));
        assert!(RefView::Id(1).validate(&view_list));
        assert!(!RefView::Id(2).validate(&view_list));
    }
}