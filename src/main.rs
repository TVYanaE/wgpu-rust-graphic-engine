use vulkan_rust_graphic_engine::{start_app, DynError};

fn main() -> Result<(), DynError> {
    start_app()?;
    Ok(())
}
