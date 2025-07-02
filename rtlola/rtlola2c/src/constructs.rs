use std::{fmt::Write, path::PathBuf};

use itertools::Itertools;
use rtlola_streamir::{
    formatter::files::{ConstructStore, FilesFormatter, Requirement},
    ir::{InputReference, OutputReference, StreamReference, Type},
};

use crate::{types::CType, CFormatter};

impl FilesFormatter for CFormatter {
    type Key = RequirementKey;

    fn get_construct_store(&self) -> &ConstructStore<Self> {
        &self.construct_store
    }

    fn overwrite(&self) -> bool {
        self.overwrite
    }
}

/// Trait to represent the definition of a function
///
/// Use `RustFormatter.call_function()` to add the function as a requirement to the store and return the expression for calling the function.
pub(crate) trait FunctionDefinition {
    /// The name of the function
    fn name(&self, _f: &CFormatter) -> String;

    /// The arguments of this function. A list of argument name-type pairs.
    fn arguments(&self, _f: &CFormatter) -> Vec<Argument> {
        Vec::new()
    }

    /// The return type of the function
    fn returns(&self, _f: &CFormatter) -> Option<CType> {
        None
    }

    /// The body of the function
    fn body(self, _f: &CFormatter) -> String;

    /// The requirement key of this function (where it is sorted in the file)
    fn key(&self) -> RequirementKey;

    /// The expression that is used to call the function with the given [Self::call_arguments].
    fn call_string(&self, f: &CFormatter, args: Vec<String>) -> String {
        format!("{}({})", self.name(f), args.join(", "))
    }

    fn file(&self, _f: &CFormatter) -> PathBuf;

    fn header_file(&self, _f: &CFormatter) -> Option<(RequirementKey, PathBuf)> {
        None
    }
}

/// Wrapper for types implementing the FunctionDefinition trait
/// to be able to generically implement the `Requirement` trait for it.
struct FD<F: FunctionDefinition>(F);

impl<F: FunctionDefinition> Requirement<CFormatter> for FD<F> {
    fn key(&self) -> RequirementKey {
        self.0.key()
    }

    fn format(self, formatter: &CFormatter) -> String {
        if let Some((key, file)) = self.0.header_file(formatter) {
            formatter.add_requirement_string(file, key, format!("{};", self.header(formatter)));
        }
        format!(
            "{header}{{\n{body}\n}}",
            header = self.header(formatter),
            body = self.0.body(formatter)
        )
    }

    fn file(&self, formatter: &CFormatter) -> PathBuf {
        self.0.file(formatter)
    }
}

impl<F: FunctionDefinition> FD<F> {
    fn header(&self, formatter: &CFormatter) -> String {
        let return_ty = self
            .0
            .returns(formatter)
            .map(|ty| formatter.c_ty(ty))
            .unwrap_or("void".into());

        let args = self
            .0
            .arguments(formatter)
            .into_iter()
            .map(|argument| formatter.argument(argument))
            .join(", ");

        format!("{return_ty} {name}({args})", name = self.0.name(formatter))
    }
}

pub(crate) enum Argument {
    Normal(String, CType),
    Array(String, CType, usize),
}

impl Argument {
    fn map_ty(self, f: impl Fn(CType) -> CType) -> Argument {
        match self {
            Argument::Normal(name, ctype) => Argument::Normal(name, f(ctype)),
            Argument::Array(name, ctype, size) => Argument::Array(name, f(ctype), size),
        }
    }

    pub(crate) fn reference(self) -> Argument {
        self.map_ty(|ty| ty.reference())
    }

    pub(crate) fn name(&self) -> &str {
        match self {
            Argument::Normal(name, _) => name,
            Argument::Array(name, _, _) => name,
        }
    }
}

impl CType {
    pub(crate) fn argument(self, name: String) -> Argument {
        Argument::Normal(name, self)
    }
}

impl CFormatter {
    fn argument(&self, arg: Argument) -> String {
        match arg {
            Argument::Normal(name, ctype) => format!("{ctype} {name}", ctype = self.c_ty(ctype)),
            Argument::Array(name, ctype, len) => {
                format!("{ctype} {name}[{len}]", ctype = self.c_ty(ctype))
            }
        }
    }

    pub(crate) fn variable_declaration(&self, arg: Argument) -> String {
        format!("{};", self.argument(arg))
    }

    pub(crate) fn variable_declaration_with_initialization(
        &self,
        arg: Argument,
        init: String,
    ) -> String {
        format!("{} = {init};", self.argument(arg))
    }
}

pub(crate) trait StructDefinition
where
    Self: Sized,
{
    fn key(&self) -> RequirementKey;
    fn file(&self, f: &CFormatter) -> PathBuf;

    fn struct_name(&self, f: &CFormatter) -> String;

    fn argument_name(&self, f: &CFormatter) -> String {
        self.struct_name(f).to_ascii_lowercase()
    }

    fn argument_name_ref(&self, f: &CFormatter) -> String {
        format!("&{}", self.argument_name(f))
    }

    fn fields(&self, f: &CFormatter) -> Vec<Argument>;
    fn as_ty(&self, f: &CFormatter) -> CType {
        CType::Other(self.struct_name(f))
    }
    fn into_argument(self, f: &CFormatter) -> Argument {
        let name = self.argument_name(f);
        let ty = self.as_ty(f);
        f.add_requirement(SD(self));
        Argument::Normal(name, ty)
    }
}

struct SD<S: StructDefinition>(S);

impl<S: StructDefinition> Requirement<CFormatter> for SD<S> {
    fn key(&self) -> <CFormatter as FilesFormatter>::Key {
        self.0.key()
    }

    fn file(&self, formatter: &CFormatter) -> PathBuf {
        self.0.file(formatter)
    }

    fn format(self, formatter: &CFormatter) -> String {
        let fields = self
            .0
            .fields(formatter)
            .into_iter()
            .fold(String::new(), |mut s, argument| {
                writeln!(&mut s, "{};", formatter.argument(argument)).unwrap();
                s
            });
        format!(
            "typedef struct {{\n{fields}}} {};",
            self.0.struct_name(formatter)
        )
    }
}

struct Import(&'static str, PathBuf, bool);

impl Requirement<CFormatter> for Import {
    fn key(&self) -> RequirementKey {
        RequirementKey::Import(self.0)
    }

    fn file(&self, _formatter: &CFormatter) -> PathBuf {
        self.1.clone()
    }

    fn format(self, _formatter: &CFormatter) -> String {
        if self.2 {
            format!("#include \"{}.h\"", self.0)
        } else {
            format!("#include <{}.h>", self.0)
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RequirementKey {
    Import(&'static str),

    // Monitor File
    SyncAccess(StreamReference),
    OffsetAccess(StreamReference),
    GetAccess(StreamReference),
    IsFreshAccess(StreamReference),
    Input(InputReference),
    Eval(OutputReference, usize),
    Spawn(OutputReference),
    Close(OutputReference),
    Shift(StreamReference),
    DynamicGuard(usize),
    NewVerdict,
    ClearActivation,
    CycleFunction,
    ReadField(Type),
    ReadEvent,
    InitMemory,
    PrintVerdict,
    Main,

    // Header File
    StaticString(usize),
    TupleStruct(Vec<Type>),
    StaticStreamMemory(StreamReference),
    DynamicStreamMemory(StreamReference),
    MemoryStruct,
    InitMemoryHeader,
    InternalEventStruct,
    VerdictStruct,
    PrintVerdictHeader,
    CycleHeader,
}

impl CFormatter {
    pub(crate) fn call_function<F: FunctionDefinition, S: AsRef<str>>(
        &self,
        f: F,
        args: &[S],
    ) -> String {
        let args: Vec<String> = args.iter().map(|a| a.as_ref().to_owned()).collect();
        let call = f.call_string(self, args);
        let fd = FD(f);
        self.add_requirement(fd);
        call
    }

    pub(crate) fn call_function_stmt<F: FunctionDefinition, S: AsRef<str>>(
        &self,
        f: F,
        args: &[S],
    ) -> String {
        format!("{};", self.call_function(f, args))
    }

    pub(crate) fn import(&self, file: PathBuf, s: &'static str) {
        self.add_requirement(Import(s, file, false));
    }

    pub(crate) fn import_own(&self, file: PathBuf, s: &'static str) {
        self.add_requirement(Import(s, file, true));
    }

    pub(crate) fn require_struct<S: StructDefinition>(&self, s: S) {
        self.add_requirement(SD(s));
    }

    pub(crate) fn monitor_file(&self) -> PathBuf {
        self.output_dir.join("monitor.c")
    }

    pub(crate) fn header_file(&self) -> PathBuf {
        self.output_dir.join("monitor.h")
    }
}
