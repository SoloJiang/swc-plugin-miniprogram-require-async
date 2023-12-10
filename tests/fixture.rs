use std::path::PathBuf;

use swc_common::{chain, Mark};
use swc_core::ecma::visit::as_folder;
use swc_ecma_transforms_base::resolver;
use swc_ecma_transforms_testing::test_fixture;
use swc_plugin_miniprogram_require_async::transform_dynamic_import;

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        Default::default(),
        &|_| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            as_folder(chain!(
                resolver(unresolved_mark, top_level_mark, false),
                transform_dynamic_import()
            ))
        },
        &input,
        &output,
        Default::default(),
    );
}