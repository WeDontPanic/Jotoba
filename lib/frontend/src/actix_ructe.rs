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

impl<T: FnOnce(&mut dyn Write) -> std::io::Result<()>> From<Render<T>> for actix_web::body::AnyBody {
    fn from(t: Render<T>) -> Self {
        let mut buf = Vec::new();
        match t.0(&mut buf) {
            Ok(()) => buf.into(),
            Err(_) => "Render failed".into(),
        }
    }
}
