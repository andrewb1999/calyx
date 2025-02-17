use std::rc::Rc;

use crate::traversal::{Action, Named, VisResult, Visitor};
use calyx_ir::{self as ir, LibrarySignatures};

#[derive(Default)]
/// Adds assignments from a components `reset` port to every
/// component that contains an input `reset` port.
pub struct ResetInsertion;

impl Named for ResetInsertion {
    fn name() -> &'static str {
        "reset-insertion"
    }

    fn description() -> &'static str {
        "connect component reset to sub-component reset for applicable components"
    }
}

impl Visitor for ResetInsertion {
    fn start(
        &mut self,
        comp: &mut ir::Component,
        sigs: &LibrarySignatures,
        _comps: &[ir::Component],
    ) -> VisResult {
        let builder = ir::Builder::new(comp, sigs);
        let reset = builder
            .component
            .signature
            .borrow()
            .get_with_attr(ir::BoolAttr::Reset);

        for cell_ref in builder.component.cells.iter() {
            let cell = cell_ref.borrow();
            if let Some(port) = cell.find_with_attr(ir::BoolAttr::Reset) {
                builder.component.continuous_assignments.push(
                    builder.build_assignment(
                        port,
                        Rc::clone(&reset),
                        ir::Guard::True,
                    ),
                )
            }
        }

        // we don't need to traverse control
        Ok(Action::Stop)
    }
}
