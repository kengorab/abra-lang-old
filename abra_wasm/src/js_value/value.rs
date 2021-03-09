use abra_core::vm::value::{Value, FnValue, ClosureValue, NativeFn, TypeValue, EnumValue, EnumVariantObj};
use serde::{Serializer, Serialize};

pub struct JsWrappedValue<'a>(pub &'a Value);

impl<'a> Serialize for JsWrappedValue<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use serde::ser::SerializeMap;

        match &self.0 {
            Value::Int(val) => {
                let mut obj = serializer.serialize_map(Some(2))?;
                obj.serialize_entry("kind", "int")?;
                obj.serialize_entry("value", &val)?;
                obj.end()
            }
            Value::Float(val) => {
                let mut obj = serializer.serialize_map(Some(2))?;
                obj.serialize_entry("kind", "float")?;
                obj.serialize_entry("value", &val)?;
                obj.end()
            }
            Value::Bool(val) => {
                let mut obj = serializer.serialize_map(Some(2))?;
                obj.serialize_entry("kind", "bool")?;
                obj.serialize_entry("value", &val)?;
                obj.end()
            }
            Value::Str(val) => {
                let mut obj = serializer.serialize_map(Some(2))?;
                obj.serialize_entry("kind", "str")?;
                obj.serialize_entry("value", &val)?;
                obj.end()
            }
            Value::StringObj(o) => {
                let mut obj = serializer.serialize_map(Some(2))?;
                obj.serialize_entry("kind", "stringObj")?;
                obj.serialize_entry("value", &*o.borrow()._inner)?;
                obj.end()
            }
            Value::ArrayObj(o) => {
                let mut obj = serializer.serialize_map(Some(2))?;
                obj.serialize_entry("kind", "arrayObj")?;

                let values = &*o.borrow()._inner;
                let values: Vec<JsWrappedValue> = values.iter().map(|i| JsWrappedValue(i)).collect();
                obj.serialize_entry("values", &values)?;
                obj.end()
            }
            Value::TupleObj(o) => {
                let mut obj = serializer.serialize_map(Some(2))?;
                obj.serialize_entry("kind", "tupleObj")?;

                let values = &*o.borrow();
                let values: Vec<JsWrappedValue> = values.iter().map(|i| JsWrappedValue(i)).collect();
                obj.serialize_entry("values", &values)?;
                obj.end()
            }
            Value::SetObj(o) => {
                let mut obj = serializer.serialize_map(Some(2))?;
                obj.serialize_entry("kind", "setObj")?;

                let values = &(*o.borrow())._inner;
                let values: Vec<JsWrappedValue> = values.iter().map(|i| JsWrappedValue(i)).collect();
                obj.serialize_entry("values", &values)?;
                obj.end()
            }
            Value::MapObj(o) => {
                let mut obj = serializer.serialize_map(Some(2))?;
                obj.serialize_entry("kind", "mapObj")?;

                let values = &(*o.borrow())._inner;
                let values: Vec<(JsWrappedValue, JsWrappedValue)> = values.iter().map(|(k, v)| (JsWrappedValue(k), JsWrappedValue(v))).collect();
                obj.serialize_entry("values", &values)?;
                obj.end()
            }
            Value::InstanceObj(o) => {
                let inst = &*o.borrow();

                let mut obj = serializer.serialize_map(Some(3))?;
                obj.serialize_entry("kind", "instanceObj")?;
                obj.serialize_entry("type_id", &inst.type_id)?;
                let value: Vec<JsWrappedValue> = inst.fields.iter().map(|i| JsWrappedValue(i)).collect();
                obj.serialize_entry("value", &value)?;
                obj.end()
            }
            Value::NativeInstanceObj(o) => {
                let inst = &*o.borrow();

                let mut obj = serializer.serialize_map(Some(3))?;
                obj.serialize_entry("kind", "nativeInstanceObj")?;
                obj.serialize_entry("type_id", &inst.type_id)?;

                let field_values = inst.inst.get_field_values();
                let value: Vec<JsWrappedValue> = field_values.iter().map(|i| JsWrappedValue(i)).collect();
                obj.serialize_entry("value", &value)?;
                obj.end()
            }
            Value::EnumVariantObj(o) => {
                let EnumVariantObj { enum_name, name, .. } = &*o.borrow();
                let mut obj = serializer.serialize_map(Some(3))?;
                obj.serialize_entry("kind", "type")?;
                obj.serialize_entry("enumName", &enum_name)?;
                obj.serialize_entry("name", &name)?;
                obj.end()
            }
            Value::Fn(FnValue { name: fn_name, .. }) |
            Value::Closure(ClosureValue { name: fn_name, .. }) => {
                let mut obj = serializer.serialize_map(Some(2))?;
                obj.serialize_entry("kind", "fn")?;
                obj.serialize_entry("name", &fn_name)?;
                obj.end()
            }
            Value::NativeFn(NativeFn { name: fn_name, .. }) => {
                let mut obj = serializer.serialize_map(Some(2))?;
                obj.serialize_entry("kind", "fn")?;
                obj.serialize_entry("name", &fn_name)?;
                obj.end()
            }
            Value::Type(TypeValue { name, .. }) => {
                let mut obj = serializer.serialize_map(Some(2))?;
                obj.serialize_entry("kind", "type")?;
                obj.serialize_entry("name", &name)?;
                obj.end()
            }
            Value::Enum(EnumValue { name, .. }) => {
                let mut obj = serializer.serialize_map(Some(2))?;
                obj.serialize_entry("kind", "type")?;
                obj.serialize_entry("name", &name)?;
                obj.end()
            }
            Value::Nil => {
                let mut obj = serializer.serialize_map(Some(1))?;
                obj.serialize_entry("kind", "nil")?;
                obj.end()
            }
        }
    }
}
