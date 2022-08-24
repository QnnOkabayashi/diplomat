use super::{
    Borrow, EnumDef, EnumPath, EnumVariant, IdentBuf, LifetimeEnv, LifetimeNode, LookupId,
    MaybeOwn, Method, NonOptional, OpaqueDef, OpaquePath, Optional, OutStructDef, OutStructField,
    OutStructPath, OutType, Param, ParamSelf, PrimitiveType, ReturnFallability, ReturnType,
    ReturnableStructPath, SelfType, Slice, StructDef, StructField, StructPath, Type, TypeLifetime,
    TypeLifetimes,
};
#[allow(unused_imports)] // use in docs links
use crate::hir;
use crate::{ast, Env};
use smallvec::SmallVec;
use strck_ident::IntoCk;

/// An error from lowering the AST to the HIR.
#[derive(Debug)]
pub enum LoweringError {
    /// The purpose of having this is that translating to the HIR has enormous
    /// potential for really detailed error handling and giving suggestions.
    ///
    /// Unfortunately, working out what the error enum should look like to enable
    /// this is really hard. The plan is that once the lowering code is completely
    /// written, we ctrl+F for `"LoweringError::Other"` in the lowering code, and turn every
    /// instance into an specialized enum variant, generalizing where possible
    /// without losing any information.
    Other(String),
}

/// Lowers an [`ast::Ident`]s into an [`hir::IdentBuf`].
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_ident(
    ident: &ast::Ident,
    context: &'static str,
    errors: &mut Vec<LoweringError>,
) -> Result<IdentBuf, ()> {
    match ident.as_str().ck() {
        Ok(name) => Ok(name.to_owned()),
        Err(e) => {
            errors.push(LoweringError::Other(format!(
                "Ident `{ident}` from {context} could not be turned into a Rust ident: {e}"
            )));
            Err(())
        }
    }
}

/// Lowers an AST definition.
pub(super) trait FromAstDef: Sized {
    /// Type of the AST definition that gets lowered.
    type AstDef;

    /// Lowers the AST definition into `Self`.
    ///
    /// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
    fn lower(
        ast_def: &Self::AstDef,
        lookup_id: &LookupId,
        in_path: &ast::Path,
        env: &Env,
        errors: &mut Vec<LoweringError>,
    ) -> Result<Self, ()>;

    /// Lowers multiple items at once
    fn lower_all(
        ast_defs: &[(&ast::Path, &Self::AstDef)],
        lookup_id: &LookupId,
        env: &Env,
        errors: &mut Vec<LoweringError>,
    ) -> Result<Vec<Self>, ()> {
        let mut hir_types = Ok(Vec::with_capacity(ast_defs.len()));

        for (in_path, ast_type) in ast_defs {
            let hir_type = FromAstDef::lower(*ast_type, lookup_id, in_path, env, errors);

            match (hir_type, &mut hir_types) {
                (Ok(hir_type), Ok(hir_types)) => hir_types.push(hir_type),
                _ => hir_types = Err(()),
            }
        }

        hir_types
    }
}

impl FromAstDef for EnumDef {
    type AstDef = ast::Enum;

    fn lower(
        ast_enum: &Self::AstDef,
        lookup_id: &LookupId,
        in_path: &ast::Path,
        env: &Env,
        errors: &mut Vec<LoweringError>,
    ) -> Result<Self, ()> {
        let name = lower_ident(&ast_enum.name, "enum name", errors);

        let mut variants = Ok(Vec::with_capacity(ast_enum.variants.len()));

        for (ident, discriminant, docs) in ast_enum.variants.iter() {
            let name = lower_ident(ident, "enum variant", errors);

            match (name, &mut variants) {
                (Ok(name), Ok(variants)) => {
                    variants.push(EnumVariant {
                        docs: docs.clone(),
                        name,
                        discriminant: *discriminant,
                    });
                }
                _ => variants = Err(()),
            }
        }

        let methods = lower_all_methods(&ast_enum.methods[..], lookup_id, in_path, env, errors);

        Ok(EnumDef::new(
            ast_enum.docs.clone(),
            name?,
            variants?,
            methods?,
        ))
    }
}

impl FromAstDef for OpaqueDef {
    type AstDef = ast::OpaqueStruct;

    fn lower(
        ast_opaque: &Self::AstDef,
        lookup_id: &LookupId,
        in_path: &ast::Path,
        env: &Env,
        errors: &mut Vec<LoweringError>,
    ) -> Result<Self, ()> {
        let name = lower_ident(&ast_opaque.name, "opaque name", errors);

        let methods = lower_all_methods(&ast_opaque.methods[..], lookup_id, in_path, env, errors);

        Ok(OpaqueDef::new(ast_opaque.docs.clone(), name?, methods?))
    }
}

impl FromAstDef for StructDef {
    type AstDef = ast::Struct;

    fn lower(
        ast_struct: &Self::AstDef,
        lookup_id: &LookupId,
        in_path: &ast::Path,
        env: &Env,
        errors: &mut Vec<LoweringError>,
    ) -> Result<Self, ()> {
        let name = lower_ident(&ast_struct.name, "struct name", errors);

        let fields = if ast_struct.fields.is_empty() {
            errors.push(LoweringError::Other(format!(
                "struct `{}` is a ZST because it has no fields",
                ast_struct.name
            )));
            Err(())
        } else {
            let mut fields = Ok(Vec::with_capacity(ast_struct.fields.len()));

            for (name, ty, docs) in ast_struct.fields.iter() {
                let name = lower_ident(name, "struct field name", errors);
                let ty = lower_type(ty, &ast_struct.lifetimes, lookup_id, in_path, env, errors);

                match (name, ty, &mut fields) {
                    (Ok(name), Ok(ty), Ok(fields)) => fields.push(StructField {
                        docs: docs.clone(),
                        name,
                        ty,
                    }),
                    _ => fields = Err(()),
                }
            }

            fields
        };

        let methods = lower_all_methods(&ast_struct.methods[..], lookup_id, in_path, env, errors);

        Ok(StructDef::new(
            ast_struct.docs.clone(),
            name?,
            fields?,
            methods?,
        ))
    }
}

impl FromAstDef for OutStructDef {
    type AstDef = ast::Struct;

    fn lower(
        ast_out_struct: &Self::AstDef,
        lookup_id: &LookupId,
        in_path: &ast::Path,
        env: &Env,
        errors: &mut Vec<LoweringError>,
    ) -> Result<Self, ()> {
        let name = lower_ident(&ast_out_struct.name, "out-struct name", errors);

        let fields = if ast_out_struct.fields.is_empty() {
            errors.push(LoweringError::Other(format!(
                "struct `{}` is a ZST because it has no fields",
                ast_out_struct.name
            )));
            Err(())
        } else {
            let mut fields = Ok(Vec::with_capacity(ast_out_struct.fields.len()));

            for (name, ty, docs) in ast_out_struct.fields.iter() {
                let name = lower_ident(name, "out-struct field name", errors);
                let ty = lower_out_type(
                    ty,
                    &ast_out_struct.lifetimes,
                    lookup_id,
                    in_path,
                    env,
                    errors,
                );

                match (name, ty, &mut fields) {
                    (Ok(name), Ok(ty), Ok(fields)) => fields.push(OutStructField {
                        docs: docs.clone(),
                        name,
                        ty,
                    }),
                    _ => fields = Err(()),
                }
            }

            fields
        };

        let methods =
            lower_all_methods(&ast_out_struct.methods[..], lookup_id, in_path, env, errors);

        Ok(OutStructDef::new(
            ast_out_struct.docs.clone(),
            name?,
            fields?,
            methods?,
        ))
    }
}

/// Lowers an [`ast::Method`]s an [`hir::Method`].
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_method(
    method: &ast::Method,
    lookup_id: &LookupId,
    in_path: &ast::Path,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<Method, ()> {
    let name = lower_ident(&method.name, "method name", errors);
    let full_path_name = lower_ident(&method.full_path_name, "method full_path_name", errors);

    let param_self = method
        .self_param
        .as_ref()
        .map(|self_param| {
            lower_self_param(
                self_param,
                &method.lifetime_env,
                lookup_id,
                &method.full_path_name,
                in_path,
                env,
                errors,
            )
        })
        .transpose();

    let (params, takes_writeable) = match method.params.split_last() {
        Some((last, remaining)) if last.is_writeable() => (remaining, true),
        _ => (&method.params[..], false),
    };

    let params = lower_many_params(
        params,
        &method.lifetime_env,
        lookup_id,
        in_path,
        env,
        errors,
    );

    let output = lower_return_type(
        &method.return_type,
        takes_writeable,
        &method.lifetime_env,
        lookup_id,
        in_path,
        env,
        errors,
    );

    let lifetime_env = lower_lifetime_env(&method.lifetime_env, errors);

    Ok(Method {
        docs: method.docs.clone(),
        name: name?,
        lifetime_env: lifetime_env?,
        param_self: param_self?,
        params: params?,
        output: output?,
    })
}

/// Lowers many [`ast::Method`]s into a vector of [`hir::Method`]s.
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_all_methods(
    ast_methods: &[ast::Method],
    lookup_id: &LookupId,
    in_path: &ast::Path,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<Vec<Method>, ()> {
    let mut methods = Ok(Vec::with_capacity(ast_methods.len()));

    for method in ast_methods {
        let method = lower_method(method, lookup_id, in_path, env, errors);
        match (method, &mut methods) {
            (Ok(method), Ok(methods)) => {
                methods.push(method);
            }
            _ => methods = Err(()),
        }
    }

    methods
}

/// Lowers an [`ast::TypeName`]s into a [`hir::Type`].
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_type(
    ty: &ast::TypeName,
    parent_lifetimes: &ast::LifetimeEnv,
    lookup_id: &LookupId,
    in_path: &ast::Path,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<Type, ()> {
    match ty {
        ast::TypeName::Primitive(prim) => Ok(Type::Primitive(PrimitiveType::from_ast(*prim))),
        ast::TypeName::Named(path) => match path.resolve(in_path, env) {
            ast::CustomType::Struct(strct) => {
                if let Some(tcx_id) = lookup_id.resolve_struct(strct) {
                    let lifetimes = TypeLifetimes::from_ast(parent_lifetimes, path);
                    Ok(Type::Struct(StructPath::new(lifetimes, tcx_id)))
                } else if lookup_id.resolve_out_struct(strct).is_some() {
                    errors.push(LoweringError::Other(format!("found struct in input that is marked with #[diplomat::out]: {ty} in {path}")));
                    Err(())
                } else {
                    unreachable!("struct `{}` wasn't found in the set of structs or out-structs, this is a bug.", strct.name);
                }
            }
            ast::CustomType::Opaque(_) => {
                errors.push(LoweringError::Other(format!("Opaque passed by value in input: {path}")));
                Err(())
            }
            ast::CustomType::Enum(enm) => {
                let tcx_id = lookup_id.resolve_enum(enm).expect("can't find enum in lookup map, which contains all enums from env");

                Ok(Type::Enum(EnumPath::new(tcx_id)))
            }
        },
        ast::TypeName::Reference(lifetime, mutability, ref_ty) => match ref_ty.as_ref() {
            ast::TypeName::Named(path) => match path.resolve(in_path, env) {
                ast::CustomType::Opaque(opaque) => {
                    let lifetimes = TypeLifetimes::from_ast(parent_lifetimes, path);
                    let borrow = Borrow::from_ast(parent_lifetimes, lifetime, *mutability);
                    let tcx_id = lookup_id.resolve_opaque(opaque).expect(
                        "can't find opaque in lookup map, which contains all opaques from env",
                    );

                    Ok(Type::Opaque(OpaquePath::new(
                        lifetimes,
                        Optional(false),
                        borrow,
                        tcx_id,
                    )))
                }
                _ => Err(LoweringError::Other(format!("found &T in input where T is a custom type, but not opaque. T = {ref_ty}"))),
            },
            _ => Err(LoweringError::Other(format!("found &T in input where T isn't a custom type and therefore not opaque. T = {ref_ty}"))),
        }
        .map_err(|e| errors.push(e)),
        ast::TypeName::Box(box_ty) => {
            errors.push(match box_ty.as_ref() {
                ast::TypeName::Named(path) => {
                    match path.resolve(in_path, env) {
                        ast::CustomType::Opaque(_) => LoweringError::Other(format!("found Box<T> in input where T is an opaque, but owned opaques aren't allowed in inputs. try &T instead? T = {path}")),
                        _ => LoweringError::Other(format!("found Box<T> in input where T is a custom type but not opaque. non-opaques can't be behind pointers, and opaques in inputs can't be owned. T = {path}")),
                    }
                }
                _ => LoweringError::Other(format!("found Box<T> in input where T isn't a custom type. T = {box_ty}")),
            });
            Err(())
        }
        ast::TypeName::Option(opt_ty) => {
            match opt_ty.as_ref() {
                ast::TypeName::Reference(lifetime, mutability, ref_ty) => {
                    match ref_ty.as_ref() {
                        ast::TypeName::Named(path) => match path.resolve(in_path, env) {
                            ast::CustomType::Opaque(opaque) => {
                                let lifetimes = TypeLifetimes::from_ast(parent_lifetimes, path);
                                let borrow = Borrow::from_ast(parent_lifetimes, lifetime, *mutability);
                                let tcx_id = lookup_id.resolve_opaque(opaque).expect(
                                    "can't find opaque in lookup map, which contains all opaques from env",
                                );

                                Ok(Type::Opaque(OpaquePath::new(
                                    lifetimes,
                                    Optional(false),
                                    borrow,
                                    tcx_id,
                                )))
                            }
                            _ => Err(LoweringError::Other(format!("found Option<&T> in input where T is a custom type, but it's not opaque. T = {ref_ty}"))),
                        },
                        _ => Err(LoweringError::Other(format!("found Option<&T> in input, but T isn't a custom type and therefore not opaque. T = {ref_ty}"))),
                    }
                    .map_err(|e| errors.push(e))
                }
                ast::TypeName::Box(box_ty) => {
                    // we could see whats in the box here too
                    errors.push(LoweringError::Other(format!("found Option<Box<T>> in input, but box isn't allowed in inputs. T = {box_ty}")));
                    Err(())
                }
                _ => {
                    errors.push(LoweringError::Other(format!("found Option<T> in input, where T isn't a reference but Option<T> in inputs requires that T is a reference to an opaque. T = {opt_ty}")));
                    Err(())
                }
            }
        }
        ast::TypeName::Result(_, _) => {
            errors.push(LoweringError::Other("Results can only appear as the top-level return type of methods".into()));
            Err(())
        }
        ast::TypeName::Writeable => {
            errors.push(LoweringError::Other("Writeables can only appear as the last parameter of a method".into()));
            Err(())
        }
        ast::TypeName::StrReference(lifetime) => {
            let lifetime = TypeLifetime::from_ast(parent_lifetimes, lifetime);

            Ok(Type::Slice(Slice::Str(lifetime)))
        }
        ast::TypeName::PrimitiveSlice(lifetime, mutability, prim) => {
            let borrow = Borrow::from_ast(parent_lifetimes, lifetime, *mutability);
            let prim = PrimitiveType::from_ast(*prim);

            Ok(Type::Slice(Slice::Primitive(borrow, prim)))
        }
        ast::TypeName::Unit => {
            errors.push(LoweringError::Other("Unit types can only appear as the return value of a method, or as the Ok/Err variants of a returned result".into()));
            Err(())
        }
    }
}

/// Lowers an [`ast::TypeName`]s into an [`hir::OutType`].
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_out_type(
    ty: &ast::TypeName,
    parent_lifetimes: &ast::LifetimeEnv,
    lookup_id: &LookupId,
    in_path: &ast::Path,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<OutType, ()> {
    match ty {
        ast::TypeName::Primitive(prim) => Ok(OutType::Primitive(PrimitiveType::from_ast(*prim))),
        ast::TypeName::Named(path) => match path.resolve(in_path, env) {
            ast::CustomType::Struct(strct) => {
                let lifetimes = TypeLifetimes::from_ast(parent_lifetimes, path);

                if let Some(tcx_id) = lookup_id.resolve_struct(strct) {
                    Ok(OutType::Struct(ReturnableStructPath::Struct(StructPath::new(lifetimes, tcx_id))))
                } else if let Some(tcx_id) = lookup_id.resolve_out_struct(strct) {
                    Ok(OutType::Struct(ReturnableStructPath::OutStruct(OutStructPath::new(lifetimes, tcx_id))))
                } else {
                    unreachable!("struct `{}` wasn't found in the set of structs or out-structs, this is a bug.", strct.name);
                }
            }
            ast::CustomType::Opaque(_) => {
                errors.push(LoweringError::Other(format!("Opaque passed by value in input: {path}")));
                Err(())
            }
            ast::CustomType::Enum(enm) => {
                let tcx_id = lookup_id.resolve_enum(enm).expect("can't find enum in lookup map, which contains all enums from env");

                Ok(OutType::Enum(EnumPath::new(tcx_id)))
            }
        },
        ast::TypeName::Reference(lifetime, mutability, ref_ty) => match ref_ty.as_ref() {
            ast::TypeName::Named(path) => match path.resolve(in_path, env) {
                ast::CustomType::Opaque(opaque) => {
                    let lifetimes = TypeLifetimes::from_ast(parent_lifetimes, path);
                    let borrow = Borrow::from_ast(parent_lifetimes, lifetime, *mutability);
                    let tcx_id = lookup_id.resolve_opaque(opaque).expect(
                        "can't find opaque in lookup map, which contains all opaques from env",
                    );

                    Ok(OutType::Opaque(OpaquePath::new(
                        lifetimes,
                        Optional(false),
                        MaybeOwn::Borrow(borrow),
                        tcx_id,
                    )))
                }
                _ => Err(LoweringError::Other(format!("found &T in output where T is a custom type, but not opaque. T = {ref_ty}"))),
            },
            _ => Err(LoweringError::Other(format!("found &T in output where T isn't a custom type and therefore not opaque. T = {ref_ty}"))),
        }
        .map_err(|e| errors.push(e)),
        ast::TypeName::Box(box_ty) => {
            errors.push(match box_ty.as_ref() {
                ast::TypeName::Named(path) => {
                    match path.resolve(in_path, env) {
                        ast::CustomType::Opaque(_) => LoweringError::Other(format!("found Box<T> in input where T is an opaque, but owned opaques aren't allowed in inputs. try &T instead? T = {path}")),
                        _ => LoweringError::Other(format!("found Box<T> in input where T is a custom type but not opaque. non-opaques can't be behind pointers, and opaques in inputs can't be owned. T = {path}")),
                    }
                }
                _ => LoweringError::Other(format!("found Box<T> in input where T isn't a custom type. T = {box_ty}")),
            });
            Err(())
        }
        ast::TypeName::Option(opt_ty) => {
            match opt_ty.as_ref() {
                ast::TypeName::Reference(lifetime, mutability, ref_ty) => {
                    match ref_ty.as_ref() {
                        ast::TypeName::Named(path) => match path.resolve(in_path, env) {
                            ast::CustomType::Opaque(opaque) => {
                                let lifetimes = TypeLifetimes::from_ast(parent_lifetimes, path);
                                let borrow = Borrow::from_ast(parent_lifetimes, lifetime, *mutability);
                                let tcx_id = lookup_id.resolve_opaque(opaque).expect(
                                    "can't find opaque in lookup map, which contains all opaques from env",
                                );

                                Ok(OutType::Opaque(OpaquePath::new(
                                    lifetimes,
                                    Optional(true),
                                    MaybeOwn::Borrow(borrow),
                                    tcx_id,
                                )))
                            }
                            _ => Err(LoweringError::Other(format!("found Option<&T> where T is a custom type, but it's not opaque. T = {ref_ty}"))),
                        },
                        _ => Err(LoweringError::Other(format!("found Option<&T>, but T isn't a custom type and therefore not opaque. T = {ref_ty}"))),
                    }
                    .map_err(|e| errors.push(e))
                }
                ast::TypeName::Box(box_ty) => {
                    match box_ty.as_ref() {
                        ast::TypeName::Named(path) => match path.resolve(in_path, env) {
                            ast::CustomType::Opaque(opaque) => {
                                let lifetimes = TypeLifetimes::from_ast(parent_lifetimes, path);
                                let tcx_id = lookup_id.resolve_opaque(opaque).expect(
                                    "can't find opaque in lookup map, which contains all opaques from env",
                                );

                                Ok(OutType::Opaque(OpaquePath::new(
                                    lifetimes,
                                    Optional(true),
                                    MaybeOwn::Own,
                                    tcx_id,
                                )))
                            }
                            _ => Err(LoweringError::Other(format!("found Option<Box<T>> where T is a custom type, but it's not opaque. T = {box_ty}"))),
                        }
                        _ => Err(LoweringError::Other(format!("found Option<Box<T>>, but T isn't a custom type and therefore not opaque. T = {box_ty}"))),
                    }
                    .map_err(|e| errors.push(e))
                }
                _ => {
                    errors.push(LoweringError::Other(format!("found Option<T>, where T isn't a reference but Option<T> in inputs requires that T is a reference to an opaque. T = {opt_ty}")));
                    Err(())
                }
            }
        }
        ast::TypeName::Result(_, _) => {
            errors.push(LoweringError::Other("Results can only appear as the top-level return type of methods".into()));
            Err(())
        }
        ast::TypeName::Writeable => {
            errors.push(LoweringError::Other("Writeables can only appear as the last parameter of a method".into()));
            Err(())
        }
        ast::TypeName::StrReference(lifetime) => {
            let lifetime = TypeLifetime::from_ast(parent_lifetimes, lifetime);

            Ok(OutType::Slice(Slice::Str(lifetime)))
        }
        ast::TypeName::PrimitiveSlice(lifetime, mutability, prim) => {
            let borrow = Borrow::from_ast(parent_lifetimes, lifetime, *mutability);
            let prim = PrimitiveType::from_ast(*prim);

            Ok(OutType::Slice(Slice::Primitive(borrow, prim)))
        }
        ast::TypeName::Unit => {
            errors.push(LoweringError::Other("Unit types can only appear as the return value of a method, or as the Ok/Err variants of a returned result".into()));
            Err(())
        }
    }
}

/// Lowers an [`ast::LifetimeEnv`] into an [`hir::LifetimeEnv`].
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_lifetime_env(
    lifetime_env: &ast::LifetimeEnv,
    errors: &mut Vec<LoweringError>,
) -> Result<LifetimeEnv, ()> {
    let mut nodes = Ok(SmallVec::new());

    for node in lifetime_env.nodes.iter() {
        let name = lower_ident(node.lifetime.name(), "lifetime", errors);
        match (name, &mut nodes) {
            (Ok(name), Ok(nodes)) => nodes.push(LifetimeNode::new(
                name,
                node.longer.iter().copied().collect(),
                node.shorter.iter().copied().collect(),
            )),
            _ => nodes = Err(()),
        }
    }

    nodes.map(LifetimeEnv::new)
}

/// Lowers an [`ast::SelfParam`] into an [`hir::ParamSelf`].
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_self_param(
    self_param: &ast::SelfParam,
    parent_lifetimes: &ast::LifetimeEnv,
    lookup_id: &LookupId,
    method_full_path: &ast::Ident, // for better error msg
    in_path: &ast::Path,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<ParamSelf, ()> {
    match self_param.path_type.resolve(in_path, env) {
        ast::CustomType::Struct(strct) => {
            if let Some(tcx_id) = lookup_id.resolve_struct(strct) {
                if self_param.reference.is_some() {
                    errors.push(LoweringError::Other(format!("Method `{method_full_path}` takes a reference to a struct as a self parameter, which isn't allowed")));
                    Err(())
                } else {
                    let lifetimes =
                        TypeLifetimes::from_ast(parent_lifetimes, &self_param.path_type);

                    Ok(ParamSelf::new(SelfType::Struct(StructPath::new(
                        lifetimes, tcx_id,
                    ))))
                }
            } else if lookup_id.resolve_out_struct(strct).is_some() {
                if self_param.reference.is_some() {
                    errors.push(LoweringError::Other(format!("Method `{method_full_path}` takes an out-struct as the self parameter, which isn't allowed. Also, it's behind a reference, but only opaques can be behind references")));
                } else {
                    errors.push(LoweringError::Other(format!("Method `{method_full_path}` takes an out-struct as the self parameter, which isn't allowed")));
                }
                Err(())
            } else {
                unreachable!(
                    "struct `{}` wasn't found in the set of structs or out-structs, this is a bug.",
                    strct.name
                );
            }
        }
        ast::CustomType::Opaque(opaque) => {
            let tcx_id = lookup_id.resolve_opaque(opaque).expect("opaque is in env");

            if let Some((lifetime, mutability)) = &self_param.reference {
                let lifetimes = TypeLifetimes::from_ast(parent_lifetimes, &self_param.path_type);
                let borrow = Borrow::from_ast(parent_lifetimes, lifetime, *mutability);

                Ok(ParamSelf::new(SelfType::Opaque(OpaquePath::new(
                    lifetimes,
                    NonOptional,
                    borrow,
                    tcx_id,
                ))))
            } else {
                errors.push(LoweringError::Other(format!("Method `{method_full_path}` takes an opaque by value as the self parameter, but opaques as inputs must be behind refs")));
                Err(())
            }
        }
        ast::CustomType::Enum(enm) => {
            let tcx_id = lookup_id.resolve_enum(enm).expect("enum is in env");

            Ok(ParamSelf::new(SelfType::Enum(EnumPath::new(tcx_id))))
        }
    }
}

/// Lowers an [`ast::Param`] into an [`hir::Param`].
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
///
/// Note that this expects that if there was a writeable param at the end in
/// the method, it's not passed into here.
fn lower_param(
    param: &ast::Param,
    parent_lifetimes: &ast::LifetimeEnv,
    lookup_id: &LookupId,
    in_path: &ast::Path,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<Param, ()> {
    let name = lower_ident(&param.name, "param name", errors);
    let ty = lower_type(&param.ty, parent_lifetimes, lookup_id, in_path, env, errors);

    Ok(Param::new(name?, ty?))
}

/// Lowers many [`ast::Param`]s into a vector of [`hir::Param`]s.
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
///
/// Note that this expects that if there was a writeable param at the end in
/// the method, `ast_params` was sliced to not include it. This happens in
/// `lower_method`, the caller of this function.
fn lower_many_params(
    ast_params: &[ast::Param],
    parent_lifetimes: &ast::LifetimeEnv,
    lookup_id: &LookupId,
    in_path: &ast::Path,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<Vec<Param>, ()> {
    let mut params = Ok(Vec::with_capacity(ast_params.len()));

    for param in ast_params {
        let param = lower_param(param, parent_lifetimes, lookup_id, in_path, env, errors);

        match (param, &mut params) {
            (Ok(param), Ok(params)) => {
                params.push(param);
            }
            _ => params = Err(()),
        }
    }

    params
}

/// Lowers the return type of an [`ast::Method`] into a [`hir::ReturnFallability`].
///
/// If there are any errors, they're pushed to `errors` and `Err(())` is returned.
fn lower_return_type(
    return_type: &Option<ast::TypeName>,
    takes_writeable: bool,
    parent_lifetimes: &ast::LifetimeEnv,
    lookup_id: &LookupId,
    in_path: &ast::Path,
    env: &Env,
    errors: &mut Vec<LoweringError>,
) -> Result<ReturnFallability, ()> {
    let return_type = return_type.as_ref().unwrap_or(&ast::TypeName::Unit);

    match return_type {
        ast::TypeName::Result(ok_ty, err_ty) => {
            let ok_ty = match ok_ty.as_ref() {
                ast::TypeName::Unit => {
                    if takes_writeable {
                        Ok(Some(ReturnType::Writeable))
                    } else {
                        Ok(None)
                    }
                }
                ty => lower_out_type(ty, parent_lifetimes, lookup_id, in_path, env, errors)
                    .map(|ty| Some(ReturnType::OutType(ty))),
            };

            let err_ty = lower_out_type(err_ty, parent_lifetimes, lookup_id, in_path, env, errors);

            match (ok_ty, err_ty) {
                (Ok(ok_ty), Ok(err_ty)) => Ok(ReturnFallability::Fallible(ok_ty, err_ty)),
                _ => Err(()),
            }
        }
        ast::TypeName::Unit => {
            if takes_writeable {
                Ok(ReturnFallability::Infallible(Some(ReturnType::Writeable)))
            } else {
                Ok(ReturnFallability::Infallible(None))
            }
        }
        _ => lower_out_type(
            return_type,
            parent_lifetimes,
            lookup_id,
            in_path,
            env,
            errors,
        )
        .map(|ty| ReturnFallability::Infallible(Some(ReturnType::OutType(ty)))),
    }
}