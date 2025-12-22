#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TaskType {
    // Init 
    Init,
    Shutdown,
    // Logic calc
    LogicCalculation,
    // Render pipeline
    PrepareRenderState, 
    DrawRenderState,
    // Another pipeline
    Resize,
    UnknowTask,
}
