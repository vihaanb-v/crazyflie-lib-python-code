use std::path::PathBuf;

use rtlola_streamir::{formatter::types::TypeFormatter, ir::Type};

use crate::{
    constructs::{Argument, RequirementKey},
    CFormatter, StructDefinition,
};

pub(crate) enum CType {
    Lola(Type),
    Other(String),
    Bool,
    Int,
    Reference(Box<CType>),
    Char,
}

impl CType {
    pub(crate) fn reference(self) -> Self {
        Self::Reference(Box::new(self))
    }

    pub(crate) fn lola(self) -> Type {
        match self {
            CType::Lola(ty) => ty,
            _ => unreachable!(),
        }
    }
}

impl CFormatter {
    pub(crate) fn c_ty(&self, ty: CType) -> String {
        match ty {
            CType::Lola(ty) => self.ty(ty),
            CType::Other(s) => s,
            CType::Bool => self.ty(Type::Bool),
            CType::Int => "int".into(),
            CType::Reference(inner) => format!("{}*", self.c_ty(*inner)),
            CType::Char => "char".into(),
        }
    }
}

impl TypeFormatter for CFormatter {
    type Return = String;

    fn type_int(&self, bits: u16) -> Self::Return {
        self.import(self.monitor_file(), "stdint");
        self.import(self.header_file(), "stdint");
        format!("int{bits}_t")
    }

    fn type_uint(&self, bits: u16) -> Self::Return {
        self.import(self.monitor_file(), "stdint");
        self.import(self.header_file(), "stdint");
        format!("uint{bits}_t")
    }

    fn type_bool(&self) -> Self::Return {
        self.import(self.monitor_file(), "stdbool");
        self.import(self.header_file(), "stdbool");
        "bool".into()
    }

    fn type_string(&self) -> Self::Return {
        "char*".into()
    }

    fn type_float32(&self) -> Self::Return {
        "float".into()
    }

    fn type_float64(&self) -> Self::Return {
        "double".into()
    }

    fn type_option(&self, _inner: Type) -> Self::Return {
        unimplemented!()
    }

    fn type_tuple(&self, inner: Vec<Type>) -> Self::Return {
        let tuple_struct = TupleType(inner);
        let name = tuple_struct.struct_name(self);
        self.require_struct(tuple_struct);
        name
    }

    fn type_fixed(&self, _bits: u16) -> Self::Return {
        unimplemented!()
    }

    fn type_ufixed(&self, _bits: u16) -> Self::Return {
        unimplemented!()
    }

    fn type_bytes(&self) -> Self::Return {
        unimplemented!()
    }
}

pub(crate) struct TupleType(pub(crate) Vec<Type>);

impl StructDefinition for TupleType {
    fn key(&self) -> RequirementKey {
        RequirementKey::TupleStruct(self.0.clone())
    }

    fn file(&self, f: &CFormatter) -> PathBuf {
        f.header_file()
    }

    fn struct_name(&self, f: &CFormatter) -> String {
        f.tuple_struct_name(&self.0)
    }

    fn fields(&self, f: &CFormatter) -> Vec<Argument> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, ty)| Argument::Normal(f.tuple_argument_name(i), CType::Lola(ty.to_owned())))
            .collect()
    }
}
