use calyx_ir::{self as cir};
use smallvec::SmallVec;

use crate::primitives::prim_utils::get_params;

use super::prelude::ComponentIdx;

#[derive(Debug, Clone)]
pub enum LiteralOrPrimitive {
    Literal,
    Primitive,
}

#[derive(Debug, Clone)]
pub enum CellPrototype {
    Component(ComponentIdx),
    Constant {
        value: u64,
        width: u64,
        c_type: LiteralOrPrimitive,
    },
    Register {
        width: u64,
    },
    // TODO Griffin: lots more
    Unknown(String, Box<cir::Binding>),
}

impl From<ComponentIdx> for CellPrototype {
    fn from(v: ComponentIdx) -> Self {
        Self::Component(v)
    }
}

impl CellPrototype {
    #[must_use]
    pub fn as_component(&self) -> Option<&ComponentIdx> {
        if let Self::Component(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn construct_primitive(cell: &cir::CellType) -> Self {
        if let cir::CellType::Primitive {
            name,
            param_binding,
            ..
        } = cell
        {
            let name: &str = name.as_ref();
            let params: &SmallVec<_> = param_binding;

            match name {
                "std_reg" => {
                    get_params![params;
                        width: "WIDTH"
                    ];

                    Self::Register { width }
                }

                "std_const" => {
                    get_params![params;
                        value: "VALUE",
                        width: "WIDTH"
                    ];

                    Self::Constant {
                        value,
                        width,
                        c_type: LiteralOrPrimitive::Primitive,
                    }
                }

                _ => CellPrototype::Unknown(
                    name.to_string(),
                    param_binding.clone(),
                ),
            }
        } else {
            unreachable!("construct_primitive called on non-primitive cell");
        }
    }

    /// Returns `true` if the cell prototype is [`Component`].
    ///
    /// [`Component`]: CellPrototype::Component
    #[must_use]
    pub fn is_component(&self) -> bool {
        matches!(self, Self::Component(..))
    }
}
