#[derive(Debug, Clone, Copy)]
pub enum FramePhaseName {
    InputProcessing,
    LogicProcessing,
    PhysicsProcessing,
    AIProcessing,
    GPUDataProcessing,
    RenderProcessing,
}
