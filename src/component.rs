use std::fmt;
use crate::style::Style;
use std::hash::{Hash, Hasher};

pub trait Component: ComponentClone {
    fn build(&self) -> Option<Vec<Box<dyn Component>>> { None }
    fn update(&self) {}
    fn draw(&self);
    fn get_style(&self) -> Style;
}
pub trait ComponentClone {
    fn clone_box(&self) -> Box<dyn Component>;
}

impl<T> ComponentClone for T
where
    T: 'static + Component + Clone + PartialEq + Copy,
{
    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Box<dyn Component> {
        self.clone_box()
    }
}

impl Hash for Box<dyn Component> {
    fn hash<H: Hasher>(&self, state: &mut H) {
    }
}
impl PartialEq for Box<dyn Component> {
    fn eq(&self, other: &Self) -> bool {
        self.build().unwrap() == other.build().unwrap()
    }
}
impl Eq for Box<dyn Component> {}

impl fmt::Debug for Box<dyn Component> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Component ").field("build", &self.build().unwrap()).finish()
    }
}
