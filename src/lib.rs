#![feature(rustc_private)]

extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

mod bevy_indirect_system_registration;

dylint_linting::dylint_library!();

#[unsafe(no_mangle)]
pub fn register_lints(_: &rustc_session::Session, store: &mut rustc_lint::LintStore) {
    store.register_lints(&[bevy_indirect_system_registration::BEVY_INDIRECT_SYSTEM_REGISTRATION]);
    store.register_late_lint_pass(Box::new(|_| {
        Box::new(bevy_indirect_system_registration::BevyIndirectSystemRegistration)
    }));
}
