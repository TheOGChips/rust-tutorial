pub trait Draw {
    fn draw (&self);
}

/* NOTE: This restricts the vector to being homogeneous vs
 *       heterogeneous (only containing one kind of type vs any
 *       type that can implement Draw)
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T> where T: Draw {
    pub fn run (&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
 */

pub struct Screen {
    //NOTE: This is similar to the concept of duck typing
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run (&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw (&self) {}
}
