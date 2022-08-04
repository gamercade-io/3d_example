pub struct Gpu {
    pub z_buffer: ZBuffer,
}

impl Gpu {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Self {
            z_buffer: ZBuffer::new(screen_width, screen_width),
        }
    }

    pub fn clear_z_buffer(&mut self) {
        self.z_buffer.clear()
    }
}

pub struct ZBuffer {
    buffer: Box<[f32]>,
    screen_width: usize,
    screen_height: usize,
}

impl ZBuffer {
    fn new(screen_width: usize, screen_height: usize) -> Self {
        Self {
            screen_width,
            screen_height,
            buffer: (0..screen_width * screen_height)
                .map(|_| f32::INFINITY)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    fn clear(&mut self) {
        self.buffer.iter_mut().for_each(|d| *d = f32::INFINITY);
    }

    // Returns true if the value was < the target value
    // and the pixel should be drawn
    pub fn test_and_set(&mut self, x: usize, y: usize, value: f32) -> bool {
        let entry = &mut self.buffer[x + (y * self.screen_width)];

        if value < *entry {
            *entry = value;
            true
        } else {
            false
        }
    }
}
