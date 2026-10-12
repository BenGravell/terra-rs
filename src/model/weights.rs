/// Set one weight and share its change equally among its peers, respecting 0-1 bounds.
/// Separate mutable references prevent selecting the same field twice.
pub fn adjust_weight(selected: &mut f64, peers: &mut [&mut f64], value: f64) -> Result<(), String> {
    let valid = |w: f64| w.is_finite() && (0.0..=1.0).contains(&w);
    if peers.is_empty() || !valid(value) || !valid(*selected) || peers.iter().any(|w| !valid(**w)) {
        return Err("Invalid weight adjustment.".into());
    }
    *selected = value;
    // Each pass either finishes or exhausts at least one peer's available weight.
    for _ in 0..peers.len() {
        let remainder = 1. - (*selected + peers.iter().map(|w| **w).sum::<f64>());
        if remainder.abs() < 1e-12 {
            break;
        }
        let available = |w: f64| if remainder > 0. { w < 1. } else { w > 0. };
        let count = peers.iter().filter(|w| available(***w)).count();
        if count == 0 {
            break;
        }
        let share = remainder / count as f64;
        for w in peers.iter_mut() {
            if available(**w) {
                **w = (**w + share).clamp(0., 1.);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "weights_test.rs"]
mod tests;
