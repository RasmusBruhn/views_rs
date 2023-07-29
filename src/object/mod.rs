pub mod extent;

struct View<'a> {
    parent: Option<&'a View<'a>>,
    children: Vec<View<'a>>,
}