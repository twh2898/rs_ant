
use std::fmt;

#[derive(Clone, Debug)]
pub struct World {
    width: usize,
    height: usize,
    elements: Vec<Vec<bool>>,
    ant: (usize, usize),
    direction: i8,
}

impl Default for World {
    fn default() -> World {
        World {
            width: 79,
            height: 24,
            elements: Vec::new(),
            ant: (10, 10),
            direction: 1,
        }
    }
}

impl World {
    pub fn new() -> World {
        Default::default()
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }

    pub fn with_ant(mut self, x: usize, y: usize, direction: i8) -> Self {
        self.ant = (x, y);
        self.direction = direction;
        self
    }

    pub fn generate(&mut self) {
        let mut rows = Vec::new();
        for _ in 0..self.height {
            let mut col = Vec::new();
            for _ in 0..self.width {
                col.push(false);
            }
            rows.push(col);
        }
        self.elements = rows;
    }

    pub fn step(&mut self) {
        let cell = self.elements[self.ant.1][self.ant.0];
        self.elements[self.ant.1][self.ant.0] = !cell;

        self.direction -= if cell { 1 } else { -1 };
        if self.direction < 0 {
            self.direction = 3;
        } else if self.direction > 3 {
            self.direction = 0;
        }

        if self.ant.0 == 0 {
            if self.direction == 3 {
                self.direction = 1;
            }
        } else if self.ant.0 == self.width - 1 {
            if self.direction == 1 {
                self.direction = 3;
            }
        }

        if self.ant.1 == 0 {
            if self.direction == 0 {
                self.direction = 2;
            }
        } else if self.ant.1 == self.height - 1 {
            if self.direction == 2 {
                self.direction = 0;
            }
        }

        match self.direction {
            0 => self.ant.1 -= 1,
            1 => self.ant.0 += 1,
            2 => self.ant.1 += 1,
            3 => self.ant.0 -= 1,
            _ => panic!(),
        }
    }
}

impl fmt::Display for World {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i = 0usize;
        let mut j = 0usize;

        for row in self.elements.iter() {
            for col in row.iter() {
                if self.ant.0 == j && self.ant.1 == i {
                    match self.direction {
                        0 => write!(f, "^").unwrap(),
                        1 => write!(f, ">").unwrap(),
                        2 => write!(f, "_").unwrap(),
                        3 => write!(f, "<").unwrap(),
                        _ => write!(f, "¤").unwrap(),
                    }
                } else {
                    match *col {
                        true => write!(f, "█").unwrap(),
                        false => write!(f, "░").unwrap(),
                    }
                }
                j += 1;
            }
            write!(f, "\n").unwrap();
            i += 1;
            j = 0;
        }
        write!(f, "\n")
    }
}
