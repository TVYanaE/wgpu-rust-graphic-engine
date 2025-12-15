#[derive(Debug, Clone, Copy)]
pub enum FramePhaseName {
    InputProcessing,
    ResizeProcessing,
    LogicProcessing,
    PhysicsProcessing,
    AIProcessing,
    GPUDataProcessing,
    RenderProcessing,
}
