use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum EngineError {
    #[error("ECS state initialization error")]
    ECSStateInitError,

    #[error("Render state initialization error")]
    RenderStateInitError,

    #[error("There is no free entity ID")]
    FreeEntityIDError,
}

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ControlThreadPhaseManagerError {
    
    #[error("Thread channel has been closed {0}")]
    ChannelClosedError(#[from] flume::TryRecvError),
}
