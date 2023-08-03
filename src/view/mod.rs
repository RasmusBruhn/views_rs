pub mod extent;

/// A view struct containing all the information of a single view
#[derive(Clone, Debug)]
pub struct View { 
    /// A vector containing all of the children of the view, children cannot be removed only added
    children: Vec<Box<View>>,
    /// The current extent of the view, this is relative to its parent, (0, 0) to (1, 1) would be the entire parent extent
    extent: extent::Extent,
}

impl View {
    /// Creates a new view
    /// 
    /// # Examples
    /// 
    /// ```
    /// use views::view::View;
    /// 
    /// let root = View::new();
    /// ```
    pub fn new() -> Self {
        let children = Vec::new();
        let extent = extent::Extent::from_span(0.0, 0.0, 0.0, 0.0);

        Self {children, extent}
    }

    pub fn update_extent(&mut self, prev: Option<&View>, siblings: &[&View]) {

    }
}

#[derive(Clone, Copy, Debug)]
pub struct ExtentUpdate {
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
pub struct ExtentStretch {
    pub pos1: PositionType,
    pub pos2: PositionType,
}

#[derive(Clone, Copy, Debug)]
pub struct ExtentLocate {
    pub pos: AnchorPoint,
    pub size: SizeType,
}

#[derive(Clone, Copy, Debug)]
pub enum PositionType {
    Anchor(AnchorPoint),
    Set(f32),
}

#[derive(Clone, Copy, Debug)]
pub enum SizeType {
    Stretch(ExtentStretch),
    Relative(RefView),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let root = View::new();

        assert_eq!(0, root.children.len());
        assert_eq!((0.0, 0.0, 0.0, 0.0), root.extent.get_span());
    }
}