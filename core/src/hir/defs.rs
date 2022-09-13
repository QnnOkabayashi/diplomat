//! Type definitions for structs, output structs, opaque structs, and enums.

use super::{IdentBuf, Method, OutType, Type};
use crate::ast::{Docs, TypeMutability};

pub enum ReturnableStructDef<'tcx> {
    Struct(&'tcx StructDef),
    OutStruct(&'tcx OutStructDef),
}

/// Structs that can only be returned from methods.
#[derive(Debug)]
pub struct OutStructDef {
    pub docs: Docs,
    pub name: IdentBuf,
    pub fields: Vec<OutStructField>,
    pub methods: Vec<Method>,
}

/// Structs that can be either inputs or outputs in methods.
#[derive(Debug)]
pub struct StructDef {
    pub docs: Docs,
    pub name: IdentBuf,
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
#[derive(Debug)]
pub struct OpaqueDef {
    pub docs: Docs,
    pub name: IdentBuf,
    pub mutability: TypeMutability,
    pub methods: Vec<Method>,
}

/// The enum type.
#[derive(Debug)]
pub struct EnumDef {
    pub docs: Docs,
    pub name: IdentBuf,
    pub variants: Vec<EnumVariant>,
    pub methods: Vec<Method>,
}

/// A field on a [`OutStruct`]s.
#[derive(Debug)]
pub struct OutStructField {
    pub docs: Docs,
    pub name: IdentBuf,
    pub ty: OutType,
}

/// A field on a [`Struct`]s.
#[derive(Debug)]
pub struct StructField {
    pub docs: Docs,
    pub name: IdentBuf,
    pub ty: Type,
}

/// A variant of an [`Enum`].
#[derive(Debug)]
pub struct EnumVariant {
    pub docs: Docs,
    pub name: IdentBuf,
    pub discriminant: isize,
}

impl OutStructDef {
    pub(super) fn new(
        docs: Docs,
        name: IdentBuf,
        fields: Vec<OutStructField>,
        methods: Vec<Method>,
    ) -> Self {
        Self {
            docs,
            name,
            fields,
            methods,
        }
    }
}

impl StructDef {
    pub(super) fn new(
        docs: Docs,
        name: IdentBuf,
        fields: Vec<StructField>,
        methods: Vec<Method>,
    ) -> Self {
        Self {
            docs,
            name,
            fields,
            methods,
        }
    }
}

impl OpaqueDef {
    pub(super) fn new(
        docs: Docs,
        name: IdentBuf,
        mutability: TypeMutability,
        methods: Vec<Method>,
    ) -> Self {
        Self {
            docs,
            name,
            mutability,
            methods,
        }
    }
}

impl EnumDef {
    pub(super) fn new(
        docs: Docs,
        name: IdentBuf,
        variants: Vec<EnumVariant>,
        methods: Vec<Method>,
    ) -> Self {
        Self {
            docs,
            name,
            variants,
            methods,
        }
    }
}
