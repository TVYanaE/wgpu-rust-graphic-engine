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
pub enum ChunkError {
    #[error("Not Enough Space in Chunk. Chunk: {0}")]
    NotEnoughSpaceInChunk(String),

    #[error("Wrong Entity Chunk Index. Chunk: {0} ")]
    WrongEntityChunkIndex(String),

    #[error("Choosed Chunk doesnt contain entity. Chunk: {0}")]
    ChunkDoesntContainEntity(String)
}

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ArchetypeError {
    #[error("Wrong determination of free chunk. Archetype: {0}")]
    EntityAddingErrorInChunk(String)
}
