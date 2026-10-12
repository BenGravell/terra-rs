use crate::ui::test_support::harness;
use egui_kittest::kittest::Queryable;

#[test]
fn table_selection_updates_the_quality_breakdown() {
    let mut h = harness(1400.);
    h.run();
    let name = h.state().data.countries[h.state().scores[1].country]
        .name
        .clone();
    h.get_by_label(&name).click();
    h.run();
    assert_eq!(h.state().selected, Some(h.state().scores[1].country));
    assert!(
        h.query_by_label(&format!("{name} · Quality of Life"))
            .is_some()
    );
}
