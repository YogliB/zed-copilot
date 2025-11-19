mod common;

use common::TestContext;

#[test]
fn test_extension_compiles_and_loads() {
    let context = TestContext::new();
    assert_eq!(context.extension_name, "zed-copilot");
}

#[test]
fn test_extension_can_be_created_via_default() {
    let context = TestContext::default();
    assert_eq!(context.extension_name, "zed-copilot");
}

#[test]
fn test_extension_does_not_panic_on_creation() {
    let result = std::panic::catch_unwind(|| {
        let _context = TestContext::new();
    });
    assert!(
        result.is_ok(),
        "Extension context creation should not panic"
    );
}

#[test]
fn test_test_context_can_be_created() {
    let context = TestContext::new();
    assert!(!context.extension_name.is_empty());
}

#[test]
fn test_multiple_contexts_can_coexist() {
    let ctx1 = TestContext::new();
    let ctx2 = TestContext::new();
    let ctx3 = TestContext::default();
    assert_eq!(ctx1.extension_name, ctx2.extension_name);
    assert_eq!(ctx2.extension_name, ctx3.extension_name);
}

#[test]
fn test_test_context_default_implementation() {
    let context = TestContext::default();
    assert_eq!(context.extension_name, "zed-copilot");
}

#[test]
fn test_extension_name_is_consistent() {
    let ctx1 = TestContext::new();
    let ctx2 = TestContext::new();
    assert_eq!(ctx1.extension_name, ctx2.extension_name);
}
