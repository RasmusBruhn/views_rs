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

    pub fn update_extent(&mut self) {

    }
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