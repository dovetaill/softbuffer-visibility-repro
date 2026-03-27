use softbuffer_visibility_repro::{FramePlan, ReproMode, ReproPlanner};

#[test]
fn baseline_mode_presents_again_on_redundant_redraw() {
    let mut planner = ReproPlanner::new(ReproMode::Baseline);

    assert_eq!(planner.plan_redraw(), FramePlan::PresentFullFrame);
    planner.note_presented();

    assert_eq!(planner.plan_redraw(), FramePlan::PresentFullFrame);
}

#[test]
fn damage_tracked_mode_skips_redundant_redraw() {
    let mut planner = ReproPlanner::new(ReproMode::DamageTracked);

    assert_eq!(planner.plan_redraw(), FramePlan::PresentFullFrame);
    planner.note_presented();

    assert_eq!(planner.plan_redraw(), FramePlan::SkipPresent);
}

#[test]
fn damage_tracked_mode_presents_after_theme_toggle_then_skips_again() {
    let mut planner = ReproPlanner::new(ReproMode::DamageTracked);

    assert_eq!(planner.plan_redraw(), FramePlan::PresentFullFrame);
    planner.note_presented();

    planner.toggle_theme();
    assert_eq!(planner.plan_redraw(), FramePlan::PresentFullFrame);
    planner.note_presented();

    assert_eq!(planner.plan_redraw(), FramePlan::SkipPresent);
}
