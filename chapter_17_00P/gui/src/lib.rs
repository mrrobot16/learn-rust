pub struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

trait Draw {
    fn draw(&self);
}