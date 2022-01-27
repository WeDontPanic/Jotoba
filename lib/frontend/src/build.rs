use ructe::{Result, Ructe};

fn main() -> Result<()> {
    let mut ructe = Ructe::from_env()?;
    ructe.compile_templates("templates")?;
    ructe.compile_templates("templates/overlays")?;
    ructe.compile_templates("templates/overlays/page")?;
    ructe.compile_templates("templates/overlays/searchbar")?;
    ructe.compile_templates("templates/subtemplates")
}
