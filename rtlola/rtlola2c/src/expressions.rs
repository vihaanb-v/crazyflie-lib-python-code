pub(crate) mod get;
pub(crate) mod is_fresh;
pub(crate) mod offset;
pub(crate) mod sync;

use std::path::PathBuf;

use get::GetAccess;
use is_fresh::IsFresh;
use itertools::Itertools;
use offset::OffsetAccess;
use rtlola_streamir::{
    formatter::{
        expressions::{
            DefaultConstantFormatter, DefaultExprFormatter, DefaultFunctionFormatter,
            DefaultOperatorFormatter, ExprFormatter, OperatorFormatter,
        },
        files::{FilesFormatter, Requirement},
        types::TypeFormatter,
    },
    ir::{
        expressions::{Expr, Operator},
        StreamReference, Type, WindowReference,
    },
};
use sync::SyncAccess;

use crate::{
    constructs::{Argument, RequirementKey, StructDefinition},
    types::TupleType,
    CFormatter, CType, MemoryStruct,
};

impl DefaultExprFormatter for CFormatter {
    fn sync_access(&self, sr: StreamReference, parameters: Vec<Expr>) -> String {
        let parameters: Vec<_> = Some(MemoryStruct.argument_name(self))
            .into_iter()
            .chain(parameters.into_iter().map(|p| self.expr(p)))
            .collect();
        let sync = SyncAccess(sr);
        self.call_function(sync, &parameters)
    }

    fn offset_access(
        &self,
        sr: StreamReference,
        offset: u32,
        default: Expr,
        parameters: Vec<Expr>,
    ) -> String {
        assert!(parameters.is_empty());
        self.call_function(
            OffsetAccess(sr),
            &[
                MemoryStruct.argument_name(self),
                offset.to_string(),
                self.expr(default),
            ],
        )
    }

    fn hold_access(&self, sr: StreamReference, default: Expr, parameters: Vec<Expr>) -> String {
        assert!(parameters.is_empty());
        self.call_function(
            OffsetAccess(sr),
            &[
                MemoryStruct.argument_name(self),
                "0".into(),
                self.expr(default),
            ],
        )
    }

    fn get_access(&self, sr: StreamReference, default: Expr, parameters: Vec<Expr>) -> String {
        assert!(parameters.is_empty());
        self.call_function(
            GetAccess(sr),
            &[MemoryStruct.argument_name(self), self.expr(default)],
        )
    }

    fn is_fresh(&self, sr: StreamReference, parameters: Vec<Expr>) -> String {
        assert!(parameters.is_empty());
        self.call_function(IsFresh(sr), &[MemoryStruct.argument_name(self)])
    }

    fn sliding_window_access(&self, _wref: usize, _default: Option<Expr>) -> String {
        unimplemented!()
    }

    fn discrete_window_access(&self, _wref: usize, _default: Option<Expr>) -> String {
        unimplemented!()
    }

    fn instance_aggregation(&self, _wref: usize, _default: Option<Expr>) -> String {
        unimplemented!()
    }

    fn parameter_access(&self, _sr: StreamReference, _p: usize) -> String {
        unimplemented!()
    }

    fn lambda_parameter_access(&self, _wref: WindowReference, _idx: usize) -> String {
        unimplemented!()
    }

    fn cast(&self, ty: Type, expr: Expr) -> String {
        format!("({})({})", self.ty(ty), self.expr(expr))
    }

    fn tuple_access(&self, expr: Expr, i: usize) -> String {
        format!("({}).{}", self.expr(expr), self.tuple_argument_name(i))
    }

    fn tuple(&self, inner: Vec<Expr>) -> String {
        TupleType(inner.iter().map(|e| e.ty.to_owned()).collect()).construct(inner, self)
    }

    fn binary(&self, op: Operator, lhs: Expr, rhs: Expr) -> String {
        match op {
            Operator::Pow => match (&lhs.ty, &rhs.ty) {
                (Type::Float64, Type::Float64) => {
                    self.import(self.monitor_file(), "math");
                    format!("pow({}, {})", self.expr(lhs), self.expr(rhs))
                }
                _ => panic!("unsupported by current compiler"),
            },
            _ => format!("({} {} {})", self.expr(lhs), self.op(op), self.expr(rhs)),
        }
    }
}

impl DefaultConstantFormatter for CFormatter {
    fn constant_string(&self, s: String) -> String {
        let s = StaticString::new(s, self);
        let name = s.variable_name(self);
        self.add_requirement(s);
        name
    }
}

impl DefaultFunctionFormatter for CFormatter {
    fn function_sqrt(&self, _arg_tys: &[Type], return_ty: &Type) -> String {
        self.import(self.monitor_file(), "math");
        match return_ty {
            Type::Float64 => "sqrt".into(),
            ty => panic!("unsupported function: sqrt({ty:?})"),
        }
    }

    fn function_abs(&self, _arg_tys: &[Type], return_ty: &Type) -> String {
        self.import(self.monitor_file(), "math");
        match return_ty {
            Type::Float64 => "fabs".into(),
            Type::Float32 => "fabsf".into(),
            Type::Int(64) => "abs".into(),
            ty => panic!("unsupported function: abs({ty:?})"),
        }
    }

    fn function_sin(&self, _arg_tys: &[Type], return_ty: &Type) -> String {
        self.import(self.monitor_file(), "math");
        match return_ty {
            Type::Float64 => "sin".into(),
            ty => panic!("unsupported function: sin({ty:?})"),
        }
    }

    fn function_arcsin(&self, _arg_tys: &[Type], return_ty: &Type) -> String {
        self.import(self.monitor_file(), "math");
        match return_ty {
            Type::Float64 => "asin".into(),
            ty => panic!("unsupported function: asin({ty:?})"),
        }
    }

    fn function_cos(&self, _arg_tys: &[Type], return_ty: &Type) -> String {
        self.import(self.monitor_file(), "math");
        match return_ty {
            Type::Float64 => "cos".into(),
            ty => panic!("unsupported function: cos({ty:?})"),
        }
    }

    fn function_arccos(&self, _arg_tys: &[Type], return_ty: &Type) -> String {
        self.import(self.monitor_file(), "math");
        match return_ty {
            Type::Float64 => "acos".into(),
            ty => panic!("unsupported function: acos({ty:?})"),
        }
    }

    fn function_tan(&self, _arg_tys: &[Type], return_ty: &Type) -> String {
        self.import(self.monitor_file(), "math");
        match return_ty {
            Type::Float64 => "tan".into(),
            ty => panic!("unsupported function: tan({ty:?})"),
        }
    }

    fn function_arctan(&self, _arg_tys: &[Type], return_ty: &Type) -> String {
        self.import(self.monitor_file(), "math");
        match return_ty {
            Type::Float64 => "atan".into(),
            ty => panic!("unsupported function: atan({ty:?})"),
        }
    }

    fn function_min(&self, _arg_tys: &[Type], return_ty: &Type) -> String {
        self.import(self.monitor_file(), "math");
        match return_ty {
            Type::Float64 => "fmin".into(),
            Type::Float32 => "fminf".into(),
            ty => panic!("unsupported function: min({ty:?})"),
        }
    }

    fn function_max(&self, _arg_tys: &[Type], return_ty: &Type) -> String {
        self.import(self.monitor_file(), "math");
        match return_ty {
            Type::Float64 => "fmax".into(),
            Type::Float32 => "fmaxf".into(),
            ty => panic!("unsupported function: max({ty:?})"),
        }
    }
}

impl DefaultOperatorFormatter for CFormatter {}

struct StaticString(usize, String);

impl StaticString {
    fn new(str: String, f: &CFormatter) -> Self {
        let strings = f.static_strings.lock().unwrap();
        let idx = if let Some(idx) = strings.get(str.as_str()) {
            *idx
        } else {
            let mut next_idx = f.next_static_string.lock().unwrap();
            let idx = *next_idx;
            *next_idx += 1;
            idx
        };
        Self(idx, str)
    }
}

impl Requirement<CFormatter> for StaticString {
    fn key(&self) -> RequirementKey {
        RequirementKey::StaticString(self.0)
    }

    fn file(&self, formatter: &CFormatter) -> PathBuf {
        formatter.header_file()
    }

    fn format(self, formatter: &CFormatter) -> String {
        formatter.variable_declaration_with_initialization(
            Argument::Normal(self.variable_name(formatter), CType::Char.reference()),
            format!("\"{}\"", self.1),
        )
    }
}

impl StaticString {
    fn variable_name(&self, f: &CFormatter) -> String {
        f.static_str_constant_name(self.0)
    }
}

impl TupleType {
    pub(crate) fn construct(self, expr: Vec<Expr>, f: &CFormatter) -> String {
        let fields = expr
            .into_iter()
            .enumerate()
            .map(|(i, e)| format!(".{}={}", f.tuple_argument_name(i), f.expr(e)))
            .join(", ");
        let struct_name = self.struct_name(f);
        format!("({struct_name}){{{fields}}}")
    }
}
