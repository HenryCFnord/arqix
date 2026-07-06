//! Command contract: `unit new` — owned by the Template Engine
//! (arc42 chapter 5).

mod common;

use common::{assert_success, run_arqix_in, scratch_copy};

// arqix:verifies REQ-00-00-00-05
#[test]
fn unit_new_creates_a_unit_from_the_configured_template() {
    let repo = scratch_copy(
        "minimal",
        "unit_new_creates_a_unit_from_the_configured_template",
    );
    let out = run_arqix_in(&repo, &["unit", "new"]);
    assert_success(&out);
}
