use crate::generator::rust::ty::*;
use crate::generator::rust::{generate_import, ExternFuncCollector};
use crate::ir::*;
use crate::target::Acc;
use crate::target::Target::*;
use crate::type_rust_generator_struct;
use crate::utils::BlockIndex;

use super::NO_PARAMS;

type_rust_generator_struct!(TypeBoxedGenerator, IrTypeBoxed);

impl TypeRustGeneratorTrait for TypeBoxedGenerator<'_> {
    fn wire2api_body(&self) -> Acc<Option<String>> {
        let box_inner = self.ir.inner.as_ref();
        let exist_in_real_api = self.ir.exist_in_real_api;
        Acc::new(|target| match (target, self.ir.inner.as_ref()) {
            (Io, IrType::Primitive(_)) => Some(
                if exist_in_real_api {
                    "unsafe { support::box_from_leak_ptr(self) }"
                } else {
                    "unsafe { *support::box_from_leak_ptr(self) }"
                }
                .into(),
            ),
            (Wasm, IrType::Primitive(_)) => Some(
                if exist_in_real_api {
                    "unsafe { support::box_from_leak_ptr(self.as_mut()) }"
                } else {
                    "unsafe { *support::box_from_leak_ptr(self.as_mut()) }"
                }
                .into(),
            ),
            (_, IrType::Primitive(_)) => None,
            (Io, _) => Some(format!(
                "let wrap = unsafe {{ support::box_from_leak_ptr(self) }};
                Wire2Api::<{}>::wire2api(*wrap).into()",
                box_inner.rust_api_type()
            )),
            _ => None,
        })
    }

    fn wasm2api_body(&self) -> Option<std::borrow::Cow<str>> {
        (self.ir.exist_in_real_api).then(|| "Box::new(self.wire2api())".into())
    }

    fn wrapper_struct(&self) -> Option<String> {
        let src = TypeRustGenerator::new(
            *self.ir.inner.clone(),
            self.context.ir_file,
            self.context.config,
        );
        src.wrapper_struct()
    }

    fn self_access(&self, obj: String) -> String {
        format!("(*{})", obj)
    }

    fn wrap_obj(&self, obj: String) -> String {
        let src = TypeRustGenerator::new(
            *self.ir.inner.clone(),
            self.context.ir_file,
            self.context.config,
        );
        src.wrap_obj(self.self_access(obj))
    }

    fn allocate_funcs(
        &self,
        collector: &mut ExternFuncCollector,
        block_index: BlockIndex,
    ) -> Acc<Option<String>> {
        if self.ir.inner.is_primitive() {
            Acc::new(|target| {
                // let wasm = target.is_wasm();
                match target {
                    Io | Wasm => Some(collector.generate(
                        &format!("new_{}_{}", self.ir.safe_ident(), block_index),
                        [(
                            format!("value: {}", self.ir.inner.rust_wire_type(target)),
                            self.ir.inner.dart_wire_type(target),
                        )],
                        Some(&format!("*mut {}", self.ir.inner.rust_wire_type(Io))),
                        "support::new_leak_box_ptr(value)",
                        target,
                    )),
                    _ => None,
                }
            })
        } else {
            Acc {
                io: Some(collector.generate(
                    &format!("new_{}_{}", self.ir.safe_ident(), block_index),
                    NO_PARAMS,
                    Some(&[self.ir.rust_wire_modifier(Io), self.ir.rust_wire_type(Io)].concat()),
                    &format!(
                        "support::new_leak_box_ptr({}::new_with_null_ptr())",
                        self.ir.inner.rust_wire_type(Io)
                    ),
                    Io,
                )),
                ..Default::default()
            }
        }
    }

    fn imports(&self) -> Option<String> {
        generate_import(&self.ir.inner, self.context.ir_file, self.context.config)
    }
}
