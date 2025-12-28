#[derive(Debug, Clone, Copy)]
pub enum ControlThreadRequest {
    InitRequest,
    ShutdownRequest,
    LogicCalculationRequest,
    RenderRequest,
}
