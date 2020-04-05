use crate::typechecker::types::Type;
use crate::vm::value::{Value, Obj};
use crate::vm::vm::VMContext;
use std::collections::HashMap;
use std::slice::Iter;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

// Native functions must return a Value, even if they're of return type Unit.
// If their return type is Unit, they should return None//Value::Nil.
type NativeAbraFn = fn(&VMContext, Vec<Value>) -> Option<Value>;

#[derive(Clone)]
pub struct NativeFn {
    pub name: String,
    pub args: Vec<Type>,
    pub opt_args: Vec<Type>,
    pub return_type: Type,
    pub native_fn: NativeAbraFn,
}

impl Debug for NativeFn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NativeFn {{ name: {}, .. }}", self.name)
    }
}

impl PartialEq for NativeFn {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl PartialOrd for NativeFn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl NativeFn {
    pub fn invoke(&self, ctx: &VMContext, args: Vec<Value>) -> Option<Value> {
        let func = self.native_fn;
        func(ctx, args)
    }
}

lazy_static! {
    pub static ref NATIVE_FNS: Vec<NativeFn> = native_fns();
    pub static ref NATIVE_FNS_MAP: HashMap<String, &'static NativeFn> = native_fns_map();
}

fn native_fns_map() -> HashMap<String, &'static NativeFn> {
    let native_fns: Iter<NativeFn> = NATIVE_FNS.iter();

    let mut map = HashMap::new();
    for native_fn in native_fns {
        let name = native_fn.name.clone();
        map.insert(name.clone(), native_fn);
    }

    map
}

fn native_fns() -> Vec<NativeFn> {
    let mut native_fns = Vec::new();

    native_fns.push(NativeFn {
        name: "println".to_string(),
        args: vec![Type::Any],
        opt_args: vec![],
        return_type: Type::Unit,
        native_fn: println,
    });

    native_fns.push(NativeFn {
        name: "range".to_string(),
        args: vec![Type::Int, Type::Int],
        opt_args: vec![Type::Int],
        return_type: Type::Array(Box::new(Type::Int)),
        native_fn: range,
    });

    native_fns.push(NativeFn {
        name: "arrayLen".to_string(),
        args: vec![Type::Array(Box::new(Type::Any))],
        opt_args: vec![],
        return_type: Type::Int,
        native_fn: arr_len,
    });

    native_fns
}

fn println(ctx: &VMContext, args: Vec<Value>) -> Option<Value> {
    let val = args.first().unwrap();
    let print_fn = ctx.print;
    print_fn(&format!("{}", val.to_string()));
    None
}

fn range(_ctx: &VMContext, args: Vec<Value>) -> Option<Value> {
    let mut start = if let Some(Value::Int(i)) = args.get(0) { *i } else {
        panic!("range requires an Int as first argument")
    };
    let end = if let Some(Value::Int(i)) = args.get(1) { *i } else {
        panic!("range requires an Int as second argument")
    };
    let incr = match args.get(2) {
        None | Some(Value::Nil) => 1,
        Some(Value::Int(i)) => *i,
        Some(_) => panic!("range requires an Int as third argument")
    };

    let size = (end - start).abs() / incr;
    let mut values = Vec::with_capacity(size as usize);

    while start < end {
        values.push(Box::new(Value::Int(start)));
        start += incr;
    }

    Some(Value::Obj(Obj::ArrayObj { value: values }))
}

// TODO: Replace this with a method invocation when Array::length is a thing
fn arr_len(_ctx: &VMContext, args: Vec<Value>) -> Option<Value> {
    let val = if let Some(Value::Obj(Obj::ArrayObj { value })) = args.first() {
        value.len()
    } else {
        panic!("arr_len requires an Array as first argument, got {:?}", args.first())
    };
    Some(Value::Int(val as i64))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn range_returning_int_array() {
        let ctx = VMContext::default();

        // Test w/ increment of 1
        let arr = range(&ctx, vec![Value::Int(0), Value::Int(5), Value::Int(1)]);
        let expected = Some(Value::Obj(Obj::ArrayObj {
            value: vec![
                Box::new(Value::Int(0)),
                Box::new(Value::Int(1)),
                Box::new(Value::Int(2)),
                Box::new(Value::Int(3)),
                Box::new(Value::Int(4)),
            ]
        }));
        assert_eq!(expected, arr);

        // Test w/ increment of 2
        let arr = range(&ctx, vec![Value::Int(0), Value::Int(5), Value::Int(2)]);
        let expected = Some(Value::Obj(Obj::ArrayObj {
            value: vec![
                Box::new(Value::Int(0)),
                Box::new(Value::Int(2)),
                Box::new(Value::Int(4)),
            ]
        }));
        assert_eq!(expected, arr);
    }

    #[test]
    fn range_returning_single_element_int_array() {
        let ctx = VMContext::default();

        // Test w/ increment larger than range
        let arr = range(&ctx, vec![Value::Int(0), Value::Int(5), Value::Int(5)]);
        let expected = Some(Value::Obj(Obj::ArrayObj { value: vec![Box::new(Value::Int(0))] }));
        assert_eq!(expected, arr);

        // Test w/ [0, 1)
        let arr = range(&ctx, vec![Value::Int(0), Value::Int(1), Value::Int(1)]);
        let expected = Some(Value::Obj(Obj::ArrayObj { value: vec![Box::new(Value::Int(0))] }));
        assert_eq!(expected, arr);

        // Test w/ [0, 0) -> Empty array
        let arr = range(&ctx, vec![Value::Int(0), Value::Int(0), Value::Int(1)]);
        let expected = Some(Value::Obj(Obj::ArrayObj { value: vec![] }));
        assert_eq!(expected, arr);
    }
}
