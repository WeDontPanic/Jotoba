use ructe::{Result, Ructe};

fn main() -> Result<()> {
    let mut ructe = Ructe::from_env()?;
    ructe.compile_templates("templates")
}
