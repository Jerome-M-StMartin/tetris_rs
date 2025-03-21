
//Concept: abstract the double-ness of the double buffer, such
//that the user interacts with it as a single buffer.
pub(crate) struct GridBuffer2 { //a la 'Vec3', 'Vec2', etc.
    buff_a: String,
    buff_b: String,
    toggle: bool,
}

impl GridBuffer2 {
    fn new() -> GridBuffer2 {
        GridBuffer2 {
            buff_a: String::new(),
            buff_b: String::new(),
            toggle: false, //true means A is safe to write and unsafe to read
        }     
    }

    pub fn write(&mut self, lines: String) {
        match self.toggle {
            true  => { self.buff_a = lines; },
            false => { self.buff_b = lines; },
        };
    }

    pub fn read(&self) -> &String {
        match self.toggle {
            true  => { &self.buff_b },
            false => { &self.buff_a },
        }
    }

    pub fn toggle(&mut self) {
        self.toggle = !self.toggle;
    }
}

