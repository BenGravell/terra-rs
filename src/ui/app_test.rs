use super::TerraApp;

#[test]
fn no_matches_clears_selection() {
    let mut app = TerraApp::default();
    app.recompute();
    assert!(app.selected.is_some());
    app.settings.continents.clear();
    app.recompute();
    assert!(app.selected.is_none());
    assert!(app.scores.iter().all(|score| !score.matches));
}
