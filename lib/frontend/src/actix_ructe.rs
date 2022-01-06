use std::io::Write;

macro_rules! render {
    ($template:path) => (super::actix_ructe::Render(|o| $template(o)));
    ($template:path, $($arg:expr),*) => {{
        use super::actix_ructe::Render;
        Render(|o| $template(o, $($arg),*))
    }};
    ($template:path, $($arg:expr),* ,) => {{
        use super::actix_ructe::Render;
        Render(|o| $template(o, $($arg),*))
    }};
}

pub struct Render<T: FnOnce(&mut dyn Write) -> std::io::Result<()>>(pub T);

impl<T: FnOnce(&mut dyn Write) -> std::io::Result<()>> Render<T> {
    pub fn render(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.0(&mut bytes).unwrap();
        bytes
    }
}
