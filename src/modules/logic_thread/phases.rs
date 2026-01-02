// Modules
pub(super) mod despawn_phase;
pub(super) mod external_event_collecting_phase;
pub(super) mod external_event_handling_phase;
pub(super) mod postlogic_phase;
pub(super) mod prelogic_phase;
pub(super) mod prepare_render_state_phase;
pub(super) mod scene_change_phase;
pub(super) mod simulation_phase;
pub(super) mod spawn_phase;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Phase {
    ExternalEventCollectingPhase,
    SceneChangePhase,
    ExternalEventHandlingPhase,
    SpawnPhase,
    DespawnPhase,
    PrelogicPhase,
    SimulationPhase,
    PostlogicPhase,
    PrepareRenderStatePhase,
}
