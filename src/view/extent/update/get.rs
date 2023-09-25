use super::{View, Ratio};
use super::{ExtentUpdate, ExtentUpdateType, ExtentUpdateSingle, ExtentStretch, ExtentLocate, ExtentRatio, Dim, SizeType, PositionType, AnchorPoint, RefView};

impl ExtentUpdate {
    /// Retrieves the extent
    /// 
    /// # Parameters
    /// 
    /// dim: The dimension to use
    /// 
    /// siblings: The list of older siblings
    pub(crate) fn get(&self, siblings: &[Box<View>], parent_ratio: Ratio) -> (f32, f32, f32, f32) {
        // Get the x and y components
        let (x, y) = match self.x.extent_type {
            // y must be evaluated before x
            ExtentUpdateType::Ratio(_) => {
                let y = self.y.get(Dim::Y, siblings, parent_ratio, 0.0);
                let x = self.x.get(Dim::X, siblings, parent_ratio, y.1);
    
                (x, y)  
            }

            // x must be evaluated before y
            _ => {
                let x = self.x.get(Dim::X, siblings, parent_ratio, 0.0);
                let y = self.y.get(Dim::Y, siblings, parent_ratio, x.1);
    
                (x, y)  
            }
        };

        (x.0, y.0, x.1, y.1)
    }
}

impl ExtentUpdateSingle {
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
        size *= self.scale_rel;
        size += self.scale_abs;

        // Make sure size is not negative
        if size < 0.0 {
            size = 0.0;
        }

        (pos, size)
    }
}

impl ExtentUpdateType {
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
}

impl ExtentRatio {
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

        // Divide by parent ratio to make up for it
        let size = other_size / match dim {
            Dim::X => parent_ratio.get_x(),
            Dim::Y => parent_ratio.get_y(),
        };

        (pos, size)
    }
}

impl ExtentLocate {
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
}

impl SizeType {
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
}

impl ExtentStretch {
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
}

impl PositionType {
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
}

impl AnchorPoint {
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
}

impl RefView {
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
}

#[cfg(test)]
mod tests {
    use crate::view::{children::Children, extent::Extent};
    use super::*;

    fn gen_view(x: f32, y: f32, w: f32, h: f32, sibling_id: usize) -> View {
        let extent_single = ExtentUpdateSingle { scale_rel: 1.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0, extent_type: ExtentUpdateType::Locate(ExtentLocate { pos: PositionType::Set(0.0), size: SizeType::Set(1.0) }) };
        let extent_info = ExtentUpdate { x: extent_single, y: extent_single };
        let mut extent = Extent::new(extent_info);
        extent.x = x;
        extent.y = y;
        extent.w = w;
        extent.h = h;
        View { children: Children::new(None), extent: extent, sibling_id: Some(sibling_id) }
    }

    #[test]
    fn ref_view() {
        let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
        let sibling2 = gen_view(5.0, 6.0, 7.0, 8.0, 1);
        let siblings = vec![Box::new(sibling1), Box::new(sibling2)];
        
        let ref_view_prev = RefView::Prev;
        assert_eq!((5.0, 7.0), ref_view_prev.get(Dim::X, &siblings));
        assert_eq!((6.0, 8.0), ref_view_prev.get(Dim::Y, &siblings));

        let ref_view_id = RefView::Id(0);
        assert_eq!((1.0, 3.0), ref_view_id.get(Dim::X, &siblings));
        assert_eq!((2.0, 4.0), ref_view_id.get(Dim::Y, &siblings));
    }

    #[test]
    fn anchor_point() {
        let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
        let siblings = vec![Box::new(sibling1)];

        let anchor_point_1 = AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 };
        assert_eq!(1.0, anchor_point_1.get(Dim::X, &siblings));

        let anchor_point_2 = AnchorPoint { ref_view: RefView::Prev, ref_point: 1.0 };
        assert_eq!(4.0, anchor_point_2.get(Dim::X, &siblings));
    }

    #[test]
    fn position_type() {
        let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
        let siblings = vec![Box::new(sibling1)];

        let position_type_anchor = PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.0 });
        assert_eq!(1.0, position_type_anchor.get(Dim::X, &siblings));

        let position_type_set = PositionType::Set(11.0);
        assert_eq!(11.0, position_type_set.get(Dim::X, &siblings));
    }

    #[test]
    fn extent_stretch() {
        let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
        let siblings = vec![Box::new(sibling1)];

        let extent_stretch = ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.5 }), pos2: PositionType::Set(6.0) };
        assert_eq!((2.5, 3.5), extent_stretch.get(Dim::X, &siblings));
    }

    #[test]
    fn size_type() {
        let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
        let siblings = vec![Box::new(sibling1)];

        let size_type_set = SizeType::Set(5.0);
        assert_eq!(5.0, size_type_set.get(Dim::X, &siblings));

        let size_type_relative = SizeType::Relative(RefView::Prev);
        assert_eq!(4.0, size_type_relative.get(Dim::Y, &siblings));

        let extent_stretch = ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.5 }), pos2: PositionType::Set(6.0) };
        let size_type_stretch = SizeType::Stretch(extent_stretch);
        assert_eq!(3.5, size_type_stretch.get(Dim::X, &siblings));
    }

    #[test]
    fn extent_locate() {
        let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
        let siblings = vec![Box::new(sibling1)];

        let extent_locate = ExtentLocate { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.5 }), size: SizeType::Set(7.0) };
        assert_eq!((2.5, 7.0), extent_locate.get(Dim::X, &siblings));
    }

    #[test]
    fn extent_ratio() {
        let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
        let siblings = vec![Box::new(sibling1)];

        let extent_ratio = ExtentRatio { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.5 }) };
        assert_eq!((2.5, 20.0), extent_ratio.get(Dim::X, &siblings, Ratio::new(2.0, 8.0).unwrap(), 5.0));
    }

    #[test]
    fn extent_update_type() {
        let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
        let siblings = vec![Box::new(sibling1)];

        let extent_locate = ExtentLocate { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.5 }), size: SizeType::Set(7.0) };
        let extent_update_type_locate = ExtentUpdateType::Locate(extent_locate);
        assert_eq!((2.5, 7.0), extent_update_type_locate.get(Dim::X, &siblings, Ratio::new(2.0, 8.0).unwrap(), 5.0));

        let extent_stretch = ExtentStretch { pos1: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.5 }), pos2: PositionType::Set(6.0) };
        let extent_update_type_stretch = ExtentUpdateType::Stretch(extent_stretch);
        assert_eq!((2.5, 3.5), extent_update_type_stretch.get(Dim::X, &siblings, Ratio::new(2.0, 8.0).unwrap(), 5.0));

        let extent_ratio = ExtentRatio { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.5 }) };
        let extent_update_type_ratio = ExtentUpdateType::Ratio(extent_ratio);
        assert_eq!((2.5, 20.0), extent_update_type_ratio.get(Dim::X, &siblings, Ratio::new(2.0, 8.0).unwrap(), 5.0));
    }

    #[test]
    fn extent_update_single() {
        let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
        let siblings = vec![Box::new(sibling1)];

        let extent_ratio = ExtentRatio { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.5 }) };
        let extent_update_single = ExtentUpdateSingle { extent_type: ExtentUpdateType::Ratio(extent_ratio), offset_rel: 0.5, offset_abs: 3.0, scale_rel: 0.2, scale_abs: -1.0 };
        assert_eq!((15.5, 3.0), extent_update_single.get(Dim::X, &siblings, Ratio::new(2.0, 8.0).unwrap(), 5.0));
    }

    #[test]
    fn extent_update() {
        let sibling1 = gen_view(1.0, 2.0, 3.0, 4.0, 0);
        let siblings = vec![Box::new(sibling1)];

        let extent_ratio = ExtentRatio { pos: PositionType::Anchor(AnchorPoint { ref_view: RefView::Prev, ref_point: 0.5 }) };
        let extent_update_single_x = ExtentUpdateSingle { extent_type: ExtentUpdateType::Ratio(extent_ratio), scale_rel: 1.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 };

        let extent_locate = ExtentLocate { pos: PositionType::Set(5.0), size: SizeType::Set(2.0) };
        let extent_update_single_y = ExtentUpdateSingle { extent_type: ExtentUpdateType::Locate(extent_locate), scale_rel: 1.0, scale_abs: 0.0, offset_rel: 0.0, offset_abs: 0.0 };

        let extent_update = ExtentUpdate { x: extent_update_single_x, y: extent_update_single_y };
        assert_eq!((2.5, 5.0, 8.0, 2.0), extent_update.get(&siblings, Ratio::new(1.0, 4.0).unwrap()));

        let extent_update_invert = ExtentUpdate { x: extent_update_single_y, y: extent_update_single_x };
        assert_eq!((5.0, 4.0, 2.0, 0.5), extent_update_invert.get(&siblings, Ratio::new(1.0, 4.0).unwrap()));
    }
}