//! Type definitions for structs, output structs, opaque structs, and enums.

use super::{IdentBuf, Method, Type, TypeKind, TypeLifetimes};
use crate::ast::Docs;

/// Structs that can only be returned from methods.
pub struct OutStruct {
    pub docs: Docs,
    pub name: IdentBuf,
    pub lifetimes: TypeLifetimes,
    pub fields: Vec<OutStructField>,
    pub methods: Vec<Method>,
}

/// Structs that can be either inputs or outputs in methods.
pub struct Struct {
    pub docs: Docs,
    pub name: IdentBuf,
    pub lifetimes: TypeLifetimes,
    pub fields: Vec<StructField>,
    pub methods: Vec<Method>,
}

/// A struct whose contents are opaque across the FFI boundary, and can only
/// cross when behind a pointer.
///
/// All opaques can be inputs or outputs when behind a reference, but owned
/// opaques can only be returned since there isn't a general way for most languages
/// to give up ownership.
///
/// A struct marked with `#[diplomat::opaque]`.
pub struct Opaque {
    pub docs: Docs,
    pub name: IdentBuf,
    pub lifetimes: TypeLifetimes,
    pub methods: Vec<Method>,
}

/// The enum type.
pub struct Enum {
    pub docs: Docs,
    pub name: IdentBuf,
    pub variants: Vec<EnumVariant>,
    pub methods: Vec<Method>,
}

/// A field on a [`OutStruct`]s.
pub struct OutStructField {
    pub docs: Docs,
    pub name: IdentBuf,
    pub ty: TypeKind,
}

/// A field on a [`Struct`]s.
pub struct StructField {
    pub docs: Docs,
    pub name: IdentBuf,
    pub ty: Type,
}

/// A variant of an [`Enum`].
pub struct EnumVariant {
    pub docs: Docs,
    pub name: IdentBuf,
    pub discriminant: isize,
}
