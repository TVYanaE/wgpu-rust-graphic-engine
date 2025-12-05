use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum EngineError {
    #[error("ECS state initialization error")]
    ECSStateInitError,

    #[error("Render state initialization error")]
    RenderStateInitError,
}
