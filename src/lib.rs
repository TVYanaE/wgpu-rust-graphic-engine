use std::{
    error::Error
};



pub type DynError = Box<dyn Error + Send + Sync + 'static>;



pub fn start_app() -> Result<(), DynError> {
    
    Ok(())
}
