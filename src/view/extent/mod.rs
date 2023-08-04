pub mod update;

/// Defines the extent of a view
#[derive(Clone, Copy, Debug)]
pub struct Extent {
    /// The x-position of the upper left corner 
    x: f32, 
    /// The y-position of the upper left corner 
    y: f32, 
    /// The width
    w: f32, 
    /// The height
    h: f32,
    // The update information
    update_info: update::ExtentUpdate,
}

impl Extent {
    pub(crate)

    /// Creates an extent with coordinate of the upper left corner and its size.
    /// w and h are set to 0 if they are negative.
    /// 
    /// # Parameters
    /// 
    /// x: The x-position of the upper left corner
    /// y: The y-position of the upper left corner
    /// w: The width of the extent
    /// h: The height of the extent
    /// 
    /// # Examples
    /// 
    /// Create an extent spanning (0.25, 0.25) to (0.75, 0.75)
    /// ```
    /// use views::view::extent::Extent;
    /// 
    /// let extent = Extent::from_size(0.25, 0.25, 0.5, 0.5); 
    /// ```
    pub fn from_size(x: f32, y: f32, mut w: f32, mut h: f32) -> Self {
        // Make sure width and height are not negative
        if w < 0.0 {
            w = 0.0;
        }
        if h < 0.0 {
            h = 0.0;
        }
        
        Self { x, y, w, h }
    }

    /// Creates an extent with coordinate of the upper left and lower right corner.
    /// w and h are set to 0 if they are negative.
    /// 
    /// # Parameters
    /// 
    /// x1: The x-position of the upper left corner
    /// y1: The y-position of the upper left corner
    /// x2: The x-position of the lower right corner
    /// y2: The y-position of the lower right corner
    /// 
    /// # Examples
    /// 
    /// Create an extent spanning (0.25, 0.25) to (0.75, 0.75)
    /// ```
    /// use views::view::extent::Extent;
    /// 
    /// let extent = Extent::from_span(0.25, 0.25, 0.75, 0.75); 
    /// ```
    pub fn from_span(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self::from_size(x1, y1, x2 - x1, y2 - y1)
    }

    /// Tests whether a point (x, y) is within the extent.
    /// Including (x1, y1) but excluding (x2, y2)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use views::view::extent::Extent;
    /// 
    /// let extent = Extent::from_span(0.25, 0.25, 0.75, 0.75);
    /// 
    /// // Inside
    /// assert!(extent.contained(0.5, 0.5));
    /// 
    /// // Outside
    /// assert!(!extent.contained(1.0, 0.0));
    /// 
    /// // On the edges
    /// assert!(extent.contained(0.25, 0.5));
    /// assert!(extent.contained(0.5, 0.25));
    /// assert!(!extent.contained(0.75, 0.5));
    /// assert!(!extent.contained(0.5, 0.75));
    /// ```
    pub fn contained(&self, x: f32, y: f32) -> bool {
        x >= self.x && y >= self.y && x < self.x + self.w && y < self.y + self.h
    }
}

#[cfg(test)]
impl Extent {
    /// Returns the size as (x, y, w, h), see from_size for further explanation
    pub(crate) fn get_size(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }

    // Returns the span as (x1, y1, x2, y2), see from_span for further explanation
    pub(crate) fn get_span(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.x + self.w, self.y + self.h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_size() {
        let extent = Extent::from_size(0.1, 0.2, 0.5, 0.8);
        assert_eq!((0.1, 0.2, 0.5, 0.8), extent.get_size());
    }

    #[test]
    fn from_size_lowwidth() {
        let extent = Extent::from_size(0.1, 0.2, -0.5, 0.8);
        assert_eq!((0.1, 0.2, 0.0, 0.8), extent.get_size());
    }

    #[test]
    fn from_size_lowheight() {
        let extent = Extent::from_size(0.1, 0.2, 0.5, -0.8);
        assert_eq!((0.1, 0.2, 0.5, 0.0), extent.get_size());
    }

    #[test]
    fn from_span() {
        let extent = Extent::from_span(0.1, 0.2, 0.5, 0.9);
        assert_eq!((0.1, 0.2, 0.5, 0.9), extent.get_span());
    }

    #[test]
    fn from_span_lowwidth() {
        let extent = Extent::from_span(0.1, 0.2, 0.0, 0.9);
        assert_eq!((0.1, 0.2, 0.1, 0.9), extent.get_span());
    }

    #[test]
    fn from_span_lowheight() {
        let extent = Extent::from_span(0.1, 0.2, 0.5, 0.1);
        assert_eq!((0.1, 0.2, 0.5, 0.2), extent.get_span());
    }

    #[test]
    fn contained_inside() {
        let extent = Extent::from_size(0.25, 0.25, 0.5, 0.5);
        assert!(extent.contained(0.5, 0.5));
    }

    #[test]
    fn contained_outside() {
        let extent = Extent::from_size(0.25, 0.25, 0.5, 0.5);
        assert!(!extent.contained(1.0, 0.0));
    }

    #[test]
    fn contained_edge() {
        let extent = Extent::from_size(0.25, 0.25, 0.5, 0.5);
        assert!(extent.contained(0.25, 0.5));
        assert!(extent.contained(0.5, 0.25));
        assert!(!extent.contained(0.75, 0.5));
        assert!(!extent.contained(0.5, 0.75));
    }
}
