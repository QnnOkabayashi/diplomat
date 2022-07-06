
use std::collections::BTreeMap;

use crate::Env;

use super::{Docs, Ident, LifetimeEnv, Method, PrimitiveType, Lifetime, Mutability, Path, ModSymbol, CustomType, OpaqueStruct, Enum, TypeName};

pub struct Environment {
    inputs: BTreeMap<InputPath, InputStruct>,
    outputs: BTreeMap<OutputPath, OutputStruct>,
    input_outputs: BTreeMap<InputOutputPath, InputOutputStruct>,
    opaques: BTreeMap<OpaquePath, OpaqueStruct>,
    enums: BTreeMap<EnumPath, Enum>,
}

impl Environment {
    pub fn from_env(env: &Env) -> Self {
        let mut table = BTreeMap::new();
        // first read in all the paths
        // then create a table with each path and a bool for "input" and "output"
        for (path, _, symbol) in env.iter_items() {
            if let ModSymbol::CustomType(custom) = symbol {
                match custom {
                    CustomType::Struct(_) => {
                        table.insert(path, (false, false));
                    }
                    CustomType::Opaque(_) => todo!(),
                    CustomType::Enum(_) => todo!(),
                }
            }
        }

        for (path, ident, symbol) in env.iter_items() {
            if let ModSymbol::CustomType(custom) = symbol {
                for method in custom.methods() {
                    if method.self_param.is_some() {
                        table.get_mut(path).unwrap().0 = true;
                    }
                    for input in method.params.iter() {
                        if let TypeName::Named(ref named) = input.ty {
                            table.get_mut(named.path()).unwrap().0 = true;
                        }
                        // mark type recursively
                    }


                    if let Some(ref return_type) = method.return_type {
                        if let TypeName::Named(ref named) = return_type {
                            table.get_mut(named.path()).unwrap().1 = true;
                        }
                        // mark type recursively
                    }
                }
            }
            // go through all the inputs of each method recursively, marking each 
            //   custom type as input=true
            // similar story for output types and output=true
            // if there are types that aren't input or output types, they're never used so complain
            // Then look at each struct's fields (nonrecursively) and verify that all other
            // fields that aren't recursive also fit criteria.
            // if some don't fit (e.g. owned ptr on input type), complain and and show a nice
            // error message.
        }

        let mut inputs = BTreeMap::new();
        let mut outputs = BTreeMap::new();
        let mut input_outputs = BTreeMap::new();
        for (path, (is_input, is_output)) in table {
            match (is_input, is_output) {
                (true, true) => input_outputs.insert(InputOutputPath(path.clone(), todo!())),
                (true, false) => inputs.insert(InputPath(path.clone()), todo!()),
                (false, true) => outputs.insert(OutputPath(path.clone()), todo!()),
                (false, false) => panic!("unused type"),
            };
        }

        Environment {
            inputs,
            outputs,
            input_outputs,
            opaques: todo!(),
            enums: todo!(),
        }
    }

    fn input_type(&self, path: &InputPath) -> &InputStruct {
        self.inputs.get(path).unwrap()
    }

    fn output_type(&self, path: &OutputPath) -> &OutputStruct {
        self.outputs.get(path).unwrap()
    }

    fn input_output_type(&self, path: &InputOutputPath) -> &InputOutputStruct {
        self.input_outputs.get(path).unwrap()
    }

    fn opaque_type(&self, path: &OpaquePath) -> &OpaqueStruct {
        self.opaques.get(path).unwrap()
    }

    fn enum_type(&self, path: &EnumPath) -> &Enum {
        self.enums.get(path).unwrap()
    }
}



#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InputPath(Path);


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OutputPath(Path);


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InputOutputPath(Path);


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OpaquePath(Path);


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumPath(Path);

/// A non-opaque struct that can be an input to a function.
pub struct InputStruct {
    pub name: Ident,
    pub docs: Docs,
    pub lifetimes: LifetimeEnv,
    pub fields: Vec<(Ident, InputType, Docs)>,
    pub methods: Vec<Method>,
}

/// A non-opaque struct that can be an output to a function.
pub struct OutputStruct {
    pub name: Ident,
    pub docs: Docs,
    pub lifetimes: LifetimeEnv,
    pub fields: Vec<(Ident, OutputType, Docs)>,
    pub methods: Vec<Method>,
}

/// A non-opaque struct that can be both an input or an output to a function.
pub struct InputOutputStruct {
    pub name: Ident,
    pub docs: Docs,
    pub lifetimes: LifetimeEnv,
    pub fields: Vec<(Ident, InputOutputType, Docs)>,
    pub methods: Vec<Method>,
}

pub enum InputPathKind {
    Input(InputPath),
    InputOutput(InputOutputPath),
}

pub enum OutputPathKind {
    Output(OutputPath),
    InputOutput(InputOutputPath),
}

pub enum InputType {
    Primitive(PrimitiveType),
    Opaque(Nullability, OpaquePath), // always borrowed
    Struct(InputPathKind),
    Enum(EnumPath),
    Slice(Slice),
    Writable,
    Unit,
}

pub enum OutputType {
    Primitive(PrimitiveType),
    Opaque(Nullability, Pointer, OpaquePath),
    Struct(OutputPathKind),
    Enum(EnumPath),
    Slice(Slice),
    Result(Box<OutputType>, Box<OutputType>),
    Unit,
}

pub enum InputOutputType {
    Primitive(PrimitiveType),
    Opaque(Nullability, OpaquePath),
    Struct(InputOutputStruct),
    Enum(EnumPath),
    Slice(Slice),
    Unit,
}

pub enum Slice {
    Str(Lifetime),
    Primitive(Lifetime, Mutability, PrimitiveType),
}

pub enum Nullability {
    Nullable,
    NonNull,
}

pub enum Pointer {
    Boxed,
    Reference(Lifetime),
}

