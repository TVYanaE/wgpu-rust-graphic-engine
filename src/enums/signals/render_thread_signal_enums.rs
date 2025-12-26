#[derive(Debug, Clone, Copy)]
pub enum RenderThreadInputSignal {
    Init,
    Shutdown,
    Resize,
    DrawRenderState,
}
