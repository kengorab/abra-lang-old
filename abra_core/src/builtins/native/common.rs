use crate::vm::vm::VM;
use crate::vm::value::{Value, FnValue, ClosureValue, TypeValue, EnumValue, EnumVariantObj};
use crate::builtins::native_fns::NativeFn;
use itertools::Itertools;

pub fn invoke_fn(vm: &mut VM, fn_obj: &Value, args: Vec<Value>) -> Value {
    let res = vm.invoke_fn(args, fn_obj.clone());
    match res {
        Ok(v) => v.unwrap_or(Value::Nil),
        Err(e) => {
            eprintln!("Runtime error: {:?}", e);
            std::process::exit(1);
        }
    }
}

pub fn default_to_string_method(receiver: Option<Value>, _args: Vec<Value>, vm: &mut VM) -> Option<Value> {
    let rcv = receiver.unwrap();
    let obj = &*rcv.as_instance_obj().borrow();
    let type_name = &obj.typ.name;
    let field_names = &obj.typ.fields;
    let values = field_names.iter().zip(&obj.fields)
        .map(|(field_name, field_value)| format!("{}: {}", field_name, to_string(field_value, vm)))
        .join(", ");
    let str_val = format!("{}({})", type_name, values);

    Some(Value::new_string_obj(str_val))
}

pub fn to_string(value: &Value, vm: &mut VM) -> String {
    match value {
        Value::Int(val) => format!("{}", val),
        Value::Float(val) => format!("{}", val),
        Value::Bool(val) => format!("{}", val),
        Value::Str(val) => val.clone(),
        Value::StringObj(o) => {
            let str = &*o.borrow()._inner;
            format!("{}", str)
        }
        Value::ArrayObj(o) => {
            let arr = &*o.borrow();
            let items = arr._inner.iter().map(|v| to_string(v, vm)).join(", ");
            format!("[{}]", items)
        }
        Value::TupleObj(o) => {
            let tup = &*o.borrow();
            let items = tup.iter().map(|v| to_string(v, vm)).join(", ");
            format!("({})", items)
        }
        Value::SetObj(o) => {
            let set = &*o.borrow();
            let items = set._inner.iter().map(|v| to_string(v, vm)).join(", ");
            format!("#{{{}}}", items)
        }
        Value::MapObj(o) => {
            let map = &*o.borrow();
            let fields = map._inner.iter()
                .map(|(k, v)| {
                    let k = to_string(k, vm);
                    let v = to_string(v, vm);
                    format!("{}: {}", k, v)
                })
                .join(", ");
            format!("{{ {} }}", fields)
        },
        Value::InstanceObj(o) => {
            let o = &*o.borrow();

            let mut tostring_method = o.typ.methods.iter()
                .find(|(name, _)| name == "toString")
                .map(|(_, m)| m)
                .expect("Every instance should have at least the default toString method")
                .clone();
            tostring_method.bind_fn_value(value.clone());

            let ret = invoke_fn(vm, &tostring_method, vec![]);
            let ret = &*ret.as_string().borrow();
            ret._inner.clone()
        }
        Value::NativeInstanceObj(o) => {
            let i = &*o.borrow();
            let v = i.inst.method_to_string(vm);
            let v = &*v.as_string().borrow();
            v._inner.clone()
        }
        Value::EnumVariantObj(o) => {
            let EnumVariantObj { enum_name, name, values, .. } = &*o.borrow();
            match values {
                None => format!("{}.{}", enum_name, name),
                Some(values) => {
                    let values = values.iter()
                        .map(|v| to_string(v, vm))
                        .join(", ");
                    format!("{}.{}({})", enum_name, name, values)
                }
            }
        }
        Value::Fn(FnValue { name, .. }) |
        Value::Closure(ClosureValue { name, .. }) => format!("<func {}>", name),
        Value::NativeFn(NativeFn { name, .. }) => format!("<func {}>", name),
        Value::Type(TypeValue { name, .. }) => format!("<type {}>", name),
        Value::Enum(EnumValue { name, .. }) => format!("<enum {}>", name),
        Value::Nil => format!("None"),
    }
}
