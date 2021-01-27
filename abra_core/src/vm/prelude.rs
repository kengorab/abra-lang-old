use crate::builtins::native_value_trait::NativeValue;
use crate::builtins::native::{NativeArray, NativeMap, NativeSet, NativeString, NativeInt, NativeFloat, NativeDate};
use crate::builtins::native_fns::native_fns;
use crate::typechecker::types::Type;
use crate::vm::value::{Value, TypeValue};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct PreludeBinding {
    name: String,
    typ: Type,
    value: Value,
}

thread_local! {
  pub static PRELUDE: Prelude = Prelude::new();
  pub static PRELUDE_NUM_CONSTS: u8 = PRELUDE.with(|p| p.num_bindings()) as u8;
  pub static PRELUDE_BINDINGS: Vec<(String, Value)> = PRELUDE.with(|p| p.get_bindings());
  pub static PRELUDE_BINDING_VALUES: Vec<Value> = PRELUDE.with(|p| p.get_binding_values());
}

#[cfg(test)]
pub static PRELUDE_PRINTLN_INDEX: u8 = 0;
#[cfg(test)]
pub static PRELUDE_INT_INDEX: u8 = 7;

pub struct Prelude {
    bindings: Vec<PreludeBinding>,
    typedefs: HashMap<String, Type>,
}

impl Prelude {
    fn new() -> Self {
        let mut bindings = Vec::new();
        let mut typedefs = HashMap::new();

        for (native_fn_desc, native_fn) in native_fns() {
            let value = Value::NativeFn(native_fn);

            let name = native_fn_desc.name.to_string();
            let typ = native_fn_desc.get_fn_type();

            bindings.push(PreludeBinding { name, typ, value });
        }

        // Insert None
        bindings.push(PreludeBinding { name: "None".to_string(), typ: Type::Option(Box::new(Type::Placeholder)), value: Value::Nil });

        let prelude_types = vec![
            ("Int", Type::Int, Some(NativeInt::get_type_value())),
            ("Float", Type::Float, Some(NativeFloat::get_type_value())),
            ("Bool", Type::Bool, None),
            ("String", Type::String, Some(NativeString::get_type_value())),
            ("Unit", Type::Unit, None),
            ("Any", Type::Any, None),
            ("Array", Type::Reference("Array".to_string(), vec![Type::Generic("T".to_string())]), Some(NativeArray::get_type_value())),
            ("Map", Type::Reference("Map".to_string(), vec![Type::Generic("K".to_string()), Type::Generic("V".to_string())]), Some(NativeMap::get_type_value())),
            ("Set", Type::Reference("Set".to_string(), vec![Type::Generic("T".to_string())]), Some(NativeSet::get_type_value())),
            ("Date", Type::Reference("Date".to_string(), vec![]), Some(NativeDate::get_type_value())),
        ];
        for (type_name, typ, type_value) in prelude_types {
            let value = match type_value {
                Some(type_value) => Value::Type(type_value),
                None => Value::Type(TypeValue {
                    name: type_name.to_string(),
                    fields: vec![],
                    constructor: None,
                    methods: vec![],
                    static_fields: vec![],
                })
            };

            let binding = PreludeBinding {
                name: type_name.to_string(),
                typ: Type::Type(type_name.to_string(), Box::new(typ.clone()), false), // TODO: is_enum should not be hard-coded false
                value,
            };
            bindings.push(binding);

            typedefs.insert(type_name.to_string(), typ);
        }

        Self { bindings, typedefs }
    }

    pub fn get_bindings(&self) -> Vec<(String, Value)> {
        self.bindings.iter()
            .map(|PreludeBinding { name, value, .. }| (name.clone(), value.clone()))
            .collect()
    }

    pub fn get_binding_values(&self) -> Vec<Value> {
        self.bindings.iter()
            .map(|PreludeBinding { value, .. }| value.clone())
            .collect()
    }

    pub fn num_bindings(&self) -> usize {
        self.bindings.len()
    }

    pub fn get_binding_types(&self) -> HashMap<String, Type> {
        self.bindings.iter()
            .map(|binding| (binding.name.clone(), binding.typ.clone()))
            .collect()
    }

    pub fn get_typedefs(&self) -> HashMap<String, Type> {
        self.typedefs.iter()
            .map(|(name, typ)| (name.clone(), typ.clone()))
            .collect()
    }

    pub fn resolve_ident<S: AsRef<str>>(&self, ident: S) -> Option<Value> {
        self.bindings.iter()
            .find_map(|b| if b.name == ident.as_ref() { Some(b.value.clone()) } else { None })
    }
}
