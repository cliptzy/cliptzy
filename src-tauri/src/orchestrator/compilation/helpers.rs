use crate::orchestrator::pipeline::{emit_progress, PipelineContext, ProgressEvent};

pub(crate) fn emit_stage(ctx: &PipelineContext, stage: &str, label: &str, current: u32, total: u32) {
    emit_progress(
        &ctx.app_handle,
        &ProgressEvent {
            stage: stage.into(),
            label: label.into(),
            current,
            total,
            detail: None,
        },
    );
}
