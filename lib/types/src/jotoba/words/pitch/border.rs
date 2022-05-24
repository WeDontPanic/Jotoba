/// An HTML border for redering pitches
#[repr(u8)]
pub enum Border {
    Left,
    Right,
    Top,
    Bottom,
}

impl Border {
    #[inline]
    pub fn get_class(&self) -> char {
        match self {
            Border::Left => 'l',
            Border::Right => 'r',
            Border::Top => 't',
            Border::Bottom => 'b',
        }
    }

    #[inline]
    pub fn horizontal(high: bool) -> Border {
        if high {
            Border::Top
        } else {
            Border::Bottom
        }
    }
}

/// Helper to build Border class strings
pub struct BorderBuilder {
    inner: String,
}

impl BorderBuilder {
    #[inline]
    pub fn new(initial: Border) -> Self {
        let mut inner = String::with_capacity(3);
        inner.push(initial.get_class());
        Self { inner }
    }

    #[inline]
    pub fn add(&mut self, border: Border) {
        self.inner.push(' ');
        self.inner.push(border.get_class());
    }

    #[inline]
    pub fn build(self) -> String {
        self.inner
    }
}
