use crate::{
    encoders, ComponentSection, ComponentSectionId, Section, SectionId, TypeRef, ALIAS_KIND_OUTER,
    ALIAS_KIND_OUTER_TYPE,
};

const COMPOUND_INTERFACE_TYPE_LIST: u8 = 0x7b;
const COMPOUND_INTERFACE_TYPE_RECORD: u8 = 0x7a;
const COMPOUND_INTERFACE_TYPE_VARIANT: u8 = 0x79;
const COMPOUND_INTERFACE_TYPE_TUPLE: u8 = 0x78;
const COMPOUND_INTERFACE_TYPE_FLAGS: u8 = 0x77;
const COMPOUND_INTERFACE_TYPE_ENUM: u8 = 0x76;
const COMPOUND_INTERFACE_TYPE_UNION: u8 = 0x75;
const COMPOUND_INTERFACE_TYPE_OPTIONAL: u8 = 0x74;
const COMPOUND_INTERFACE_TYPE_EXPECTED: u8 = 0x73;
const COMPOUND_INTERFACE_TYPE_NAMED: u8 = 0x72;

const INTERFACE_TYPE_BOOL: u8 = 0x71;
const INTERFACE_TYPE_S8: u8 = 0x70;
const INTERFACE_TYPE_U8: u8 = 0x6f;
const INTERFACE_TYPE_S16: u8 = 0x6e;
const INTERFACE_TYPE_U16: u8 = 0x6d;
const INTERFACE_TYPE_S32: u8 = 0x6c;
const INTERFACE_TYPE_U32: u8 = 0x6b;
const INTERFACE_TYPE_S64: u8 = 0x6a;
const INTERFACE_TYPE_U64: u8 = 0x69;
const INTERFACE_TYPE_F32: u8 = 0x68;
const INTERFACE_TYPE_F64: u8 = 0x67;
const INTERFACE_TYPE_CHAR: u8 = 0x66;
const INTERFACE_TYPE_STRING: u8 = 0x65;

const INSTANCE_TYPEDEF_TYPE: u8 = 0x01;
const INSTANCE_TYPEDEF_ALIAS: u8 = 0x05;
const INSTANCE_TYPEDEF_EXPORT: u8 = 0x06;

const MODULE_TYPEDEF_TYPE: u8 = 0x01;
const MODULE_TYPEDEF_ALIAS: u8 = 0x05;
const MODULE_TYPEDEF_EXPORT: u8 = 0x06;
const MODULE_TYPEDEF_IMPORT: u8 = 0x02;

/// The type of a value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[repr(u8)]
pub enum ValType {
    /// The `i32` type.
    I32 = 0x7F,
    /// The `i64` type.
    I64 = 0x7E,
    /// The `f32` type.
    F32 = 0x7D,
    /// The `f64` type.
    F64 = 0x7C,
    /// The `v128` type.
    ///
    /// Part of the SIMD proposal.
    V128 = 0x7B,
    /// The `funcref` type.
    ///
    /// Part of the reference types proposal when used anywhere other than a
    /// table's element type.
    FuncRef = 0x70,
    /// The `externref` type.
    ///
    /// Part of the reference types proposal.
    ExternRef = 0x6F,
}

impl From<ValType> for u8 {
    #[inline]
    fn from(t: ValType) -> u8 {
        t as u8
    }
}

/// Represents an instance type.
#[derive(Debug, Clone, Default)]
pub struct InstanceType {
    bytes: Vec<u8>,
    num_added: u32,
}

impl InstanceType {
    /// Creates a new instance type.
    ///
    /// Instance types can only be used when encoding components.
    pub fn new() -> Self {
        Self::default()
    }

    /// Define a type in this instance type.
    ///
    /// The returned encoder must be finished before adding another definition.
    #[must_use = "the encoder must be used to encode the type"]
    pub fn ty(&mut self) -> TypeEncoder {
        self.bytes.push(INSTANCE_TYPEDEF_TYPE);
        self.num_added += 1;
        TypeEncoder(&mut self.bytes)
    }

    /// Defines an alias to an outer module's type in the instance type.
    pub fn alias_outer_type(&mut self, count: u32, index: u32) -> &mut Self {
        self.bytes.push(INSTANCE_TYPEDEF_ALIAS);
        self.bytes.push(ALIAS_KIND_OUTER);
        self.bytes.extend(encoders::u32(count));
        self.bytes.extend(encoders::u32(index));
        self.bytes.push(ALIAS_KIND_OUTER_TYPE);
        self.num_added += 1;
        self
    }

    /// Defines an export in the instance type.
    pub fn export(&mut self, name: &str, ty: TypeRef) -> &mut Self {
        self.bytes.push(INSTANCE_TYPEDEF_EXPORT);
        self.bytes.extend(encoders::str(name));
        ty.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    pub(crate) fn encode(&self, bytes: &mut Vec<u8>) {
        bytes.extend(encoders::u32(self.num_added));
        bytes.extend(self.bytes.iter().copied());
    }
}

/// Represents a module type.
#[derive(Debug, Clone, Default)]
pub struct ModuleType {
    bytes: Vec<u8>,
    num_added: u32,
}

impl ModuleType {
    /// Creates a new module type.
    ///
    /// Module types can only be used when encoding components.
    pub fn new() -> Self {
        Self::default()
    }

    /// Define a type in this module type.
    ///
    /// The returned encoder must be finished before adding another definition.
    #[must_use = "the encoder must be used to encode the type"]
    pub fn ty(&mut self) -> TypeEncoder {
        self.bytes.push(MODULE_TYPEDEF_TYPE);
        self.num_added += 1;
        TypeEncoder(&mut self.bytes)
    }

    /// Defines an alias to an outer module's type in the module type.
    pub fn alias_outer_type(&mut self, count: u32, index: u32) -> &mut Self {
        self.bytes.push(MODULE_TYPEDEF_ALIAS);
        self.bytes.push(ALIAS_KIND_OUTER);
        self.bytes.extend(encoders::u32(count));
        self.bytes.extend(encoders::u32(index));
        self.bytes.push(ALIAS_KIND_OUTER_TYPE);
        self.num_added += 1;
        self
    }

    /// Defines an export in the module type.
    pub fn export(&mut self, name: &str, ty: TypeRef) -> &mut Self {
        self.bytes.push(MODULE_TYPEDEF_EXPORT);
        self.bytes.extend(encoders::str(name));
        ty.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Defines an import in the module type.
    pub fn import(&mut self, name: &str, ty: TypeRef) -> &mut Self {
        self.bytes.push(MODULE_TYPEDEF_IMPORT);
        self.bytes.extend(encoders::str(name));
        ty.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    pub(crate) fn encode(&self, bytes: &mut Vec<u8>) {
        bytes.extend(encoders::u32(self.num_added));
        bytes.extend(self.bytes.iter().copied());
    }
}

/// Used to encode types.
#[derive(Debug)]
pub struct TypeEncoder<'a>(&'a mut Vec<u8>);

impl<'a> TypeEncoder<'a> {
    /// Define an instance type.
    pub fn instance(self, ty: &InstanceType) {
        self.0.push(0x7f);
        ty.encode(self.0);
    }

    /// Define a module type.
    pub fn module(self, ty: &ModuleType) {
        self.0.push(0x7e);
        ty.encode(self.0);
    }

    /// Define a function type.
    pub fn function<P, R>(self, params: P, results: R)
    where
        P: IntoIterator<Item = ValType>,
        P::IntoIter: ExactSizeIterator,
        R: IntoIterator<Item = ValType>,
        R::IntoIter: ExactSizeIterator,
    {
        let params = params.into_iter();
        let results = results.into_iter();

        // FIXME: this needs to be fixed for component encoding prior to merging
        self.0.push(0x60);
        self.0
            .extend(encoders::u32(u32::try_from(params.len()).unwrap()));
        self.0.extend(params.map(u8::from));
        self.0
            .extend(encoders::u32(u32::try_from(results.len()).unwrap()));
        self.0.extend(results.map(u8::from));
    }

    /// Define an adapter function type.
    ///
    /// This method can only be used for encoding components.
    pub fn adapter_function(self, params: &[(&str, InterfaceType)], result: Option<InterfaceType>) {
        self.0.push(0x7c);

        self.0
            .extend(encoders::u32(u32::try_from(params.len()).unwrap()));
        for (name, param) in params {
            self.0.extend(encoders::str(name));
            param.encode(self.0);
        }

        match result {
            Some(result) => {
                self.0.push(1);
                result.encode(self.0);
            }
            None => {
                self.0.push(0);
            }
        }
    }

    /// Define a compound type.
    ///
    /// This method can only be used for encoding components.
    ///
    /// The returned encoder must be finished before adding another type.
    #[must_use = "the encoder must be used to encode the type"]
    pub fn compound(self) -> CompoundTypeEncoder<'a> {
        CompoundTypeEncoder(self.0)
    }
}

/// Represents an interface type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceType {
    /// The type is a boolean.
    Bool,
    /// The type is a signed 8-bit integer.
    S8,
    /// The type is an unsigned 8-bit integer.
    U8,
    /// The type is a signed 16-bit integer.
    S16,
    /// The type is an unsigned 16-bit integer.
    U16,
    /// The type is a signed 32-bit integer.
    S32,
    /// The type is an unsigned 32-bit integer.
    U32,
    /// The type is a signed 64-bit integer.
    S64,
    /// The type is an unsigned 64-bit integer.
    U64,
    /// The type is a 32-bit floating point number.
    F32,
    /// The type is a 64-bit floating point number.
    F64,
    /// The type is a Unicode character.
    Char,
    /// The type is a string.
    String,
    /// The type is a compound interface type.
    ///
    /// The value is a type index to a compound type.
    Compound(u32),
}

impl InterfaceType {
    pub(crate) fn encode(&self, bytes: &mut Vec<u8>) {
        match self {
            Self::Bool => bytes.push(INTERFACE_TYPE_BOOL),
            Self::S8 => bytes.push(INTERFACE_TYPE_S8),
            Self::U8 => bytes.push(INTERFACE_TYPE_U8),
            Self::S16 => bytes.push(INTERFACE_TYPE_S16),
            Self::U16 => bytes.push(INTERFACE_TYPE_U16),
            Self::S32 => bytes.push(INTERFACE_TYPE_S32),
            Self::U32 => bytes.push(INTERFACE_TYPE_U32),
            Self::S64 => bytes.push(INTERFACE_TYPE_S64),
            Self::U64 => bytes.push(INTERFACE_TYPE_U64),
            Self::F32 => bytes.push(INTERFACE_TYPE_F32),
            Self::F64 => bytes.push(INTERFACE_TYPE_F64),
            Self::Char => bytes.push(INTERFACE_TYPE_CHAR),
            Self::String => bytes.push(INTERFACE_TYPE_STRING),
            Self::Compound(index) => bytes.extend(encoders::u32(*index)),
        }
    }
}

/// Used for encoding compound interface types.
#[derive(Debug)]
pub struct CompoundTypeEncoder<'a>(&'a mut Vec<u8>);

impl CompoundTypeEncoder<'_> {
    /// Define a list type.
    pub fn list(self, ty: InterfaceType) {
        self.0.push(COMPOUND_INTERFACE_TYPE_LIST);
        ty.encode(self.0);
    }

    /// Define a record type.
    pub fn record(self, fields: &[(&str, InterfaceType)]) {
        self.0.push(COMPOUND_INTERFACE_TYPE_RECORD);
        self.0
            .extend(encoders::u32(fields.len().try_into().unwrap()));
        for (name, ty) in fields {
            self.0.extend(encoders::str(name));
            ty.encode(self.0);
        }
    }

    /// Define a variant type.
    pub fn variant(self, cases: &[(&str, Option<InterfaceType>)]) {
        self.0.push(COMPOUND_INTERFACE_TYPE_VARIANT);
        self.0
            .extend(encoders::u32(cases.len().try_into().unwrap()));
        for (name, ty) in cases {
            self.0.extend(encoders::str(name));
            if let Some(ty) = ty {
                self.0.push(0x01);
                ty.encode(self.0);
            } else {
                self.0.push(0x00);
            }
        }
    }

    /// Define a tuple type.
    pub fn tuple(self, types: &[InterfaceType]) {
        self.0.push(COMPOUND_INTERFACE_TYPE_TUPLE);
        self.0
            .extend(encoders::u32(types.len().try_into().unwrap()));
        for ty in types {
            ty.encode(self.0);
        }
    }

    /// Define a flags type.
    pub fn flags(self, names: &[&str]) {
        self.0.push(COMPOUND_INTERFACE_TYPE_FLAGS);
        self.0
            .extend(encoders::u32(names.len().try_into().unwrap()));
        for name in names {
            self.0.extend(encoders::str(name));
        }
    }

    /// Define an enum type.
    pub fn enum_type(self, tags: &[&str]) {
        self.0.push(COMPOUND_INTERFACE_TYPE_ENUM);
        self.0.extend(encoders::u32(tags.len().try_into().unwrap()));
        for tag in tags {
            self.0.extend(encoders::str(tag));
        }
    }

    /// Define a union type.
    pub fn union(self, types: &[InterfaceType]) {
        self.0.push(COMPOUND_INTERFACE_TYPE_UNION);
        self.0
            .extend(encoders::u32(types.len().try_into().unwrap()));
        for ty in types {
            ty.encode(self.0);
        }
    }

    /// Define an optional type.
    pub fn optional(self, ty: InterfaceType) {
        self.0.push(COMPOUND_INTERFACE_TYPE_OPTIONAL);
        ty.encode(self.0);
    }

    /// Define an expected type.
    pub fn expected(self, ok: Option<InterfaceType>, error: Option<InterfaceType>) {
        self.0.push(COMPOUND_INTERFACE_TYPE_EXPECTED);
        if let Some(ok) = ok {
            self.0.push(0x01);
            ok.encode(self.0);
        } else {
            self.0.push(0x00);
        }
        if let Some(error) = error {
            self.0.push(0x01);
            error.encode(self.0);
        } else {
            self.0.push(0x00);
        }
    }

    /// Define a named type.
    pub fn named(self, name: &str, ty: InterfaceType) {
        self.0.push(COMPOUND_INTERFACE_TYPE_NAMED);
        self.0.extend(encoders::str(name));
        ty.encode(self.0);
    }
}

/// An encoder for the type section.
///
/// # Example
///
/// ```rust
/// use wasm_encoder::{Module, TypeSection, ValType};
///
/// let mut types = TypeSection::new();
///
/// types.function([ValType::I32, ValType::I32], [ValType::I64]);
///
/// let mut module = Module::new();
/// module.section(&types);
///
/// let bytes = module.finish();
/// ```
#[derive(Clone, Debug, Default)]
pub struct TypeSection {
    bytes: Vec<u8>,
    num_added: u32,
}

impl TypeSection {
    /// Create a new component type section encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of types in the section.
    pub fn len(&self) -> u32 {
        self.num_added
    }

    /// Determines if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.num_added == 0
    }

    /// Define a function type in this type section.
    ///
    /// This method can only be used when encoding modules.
    pub fn function<P, R>(&mut self, params: P, results: R) -> &mut Self
    where
        P: IntoIterator<Item = ValType>,
        P::IntoIter: ExactSizeIterator,
        R: IntoIterator<Item = ValType>,
        R::IntoIter: ExactSizeIterator,
    {
        let encoder = TypeEncoder(&mut self.bytes);
        encoder.function(params, results);
        self.num_added += 1;
        self
    }

    /// Define an instance type in this type section.
    ///
    /// This method can only be used for encoding components.
    pub fn instance(&mut self, ty: &InstanceType) -> &mut Self {
        let encoder = TypeEncoder(&mut self.bytes);
        encoder.instance(ty);
        self.num_added += 1;
        self
    }

    /// Define a module type in this type section.
    ///
    /// This method can only be used for encoding components.
    pub fn module(&mut self, ty: &ModuleType) -> &mut Self {
        let encoder = TypeEncoder(&mut self.bytes);
        encoder.module(ty);
        self.num_added += 1;
        self
    }

    /// Define an adapter function type.
    ///
    /// This method can only be used for encoding components.
    pub fn adapter_function(
        &mut self,
        params: &[(&str, InterfaceType)],
        result: Option<InterfaceType>,
    ) -> &mut Self {
        let encoder = TypeEncoder(&mut self.bytes);
        encoder.adapter_function(params, result);
        self.num_added += 1;
        self
    }

    /// Define a compound type.
    ///
    /// This method can only be used for encoding components.
    ///
    /// The returned encoder must be finished before adding another type.
    #[must_use = "the encoder must be used to encode the type"]
    pub fn compound(&mut self) -> CompoundTypeEncoder<'_> {
        let encoder = CompoundTypeEncoder(&mut self.bytes);
        self.num_added += 1;
        encoder
    }

    fn encode(&self, sink: &mut impl Extend<u8>) {
        let num_added = encoders::u32(self.num_added);
        let n = num_added.len();
        sink.extend(
            encoders::u32(u32::try_from(n + self.bytes.len()).unwrap())
                .chain(num_added)
                .chain(self.bytes.iter().copied()),
        );
    }
}

impl Section for TypeSection {
    fn id(&self) -> u8 {
        SectionId::Type.into()
    }

    fn encode<S>(&self, sink: &mut S)
    where
        S: Extend<u8>,
    {
        self.encode(sink);
    }
}

impl ComponentSection for TypeSection {
    fn id(&self) -> u8 {
        ComponentSectionId::Type.into()
    }

    fn encode<S>(&self, sink: &mut S)
    where
        S: Extend<u8>,
    {
        self.encode(sink);
    }
}
