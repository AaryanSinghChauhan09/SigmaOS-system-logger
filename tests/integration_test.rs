// Integration test for SigmaOS-system-logger
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use system_logger as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
