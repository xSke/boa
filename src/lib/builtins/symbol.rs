use crate::{
    builtins::{
        object::{Object, PROTOTYPE},
        value::{to_value, ResultValue, Value, ValueData},
    },
    exec::Interpreter,
};
use gc::{Gc, GcCell};
use rand::random;

/// https://tc39.es/ecma262/#sec-symbol-description
/// Creates Symbol instances.
///
/// Symbol instances are ordinary objects that inherit properties from the Symbol prototype object.
/// Symbol instances have a [[SymbolData]] internal slot.
/// The [[SymbolData]] internal slot is the Symbol value represented by this Symbol object.
pub fn call_symbol(_: &Value, args: &[Value], _: &mut Interpreter) -> ResultValue {
    // From an implementation and specificaition perspective Symbols are similar to Objects.
    // They have internal slots to hold the SymbolData and Description, they also have methods and a prototype.
    // So we start by creating an Object
    // TODO: Set prototype to Symbol.prototype (by changing to Object::create(), use interpreter to get Symbol.prototype)
    let mut sym_instance = Object::default();

    // Set description which should either be undefined or a string
    let desc_string = match args.get(0) {
        Some(value) => to_value(value.to_string()),
        None => Gc::new(ValueData::Undefined),
    };

    sym_instance.set_internal_slot("Description", desc_string);
    sym_instance.set_internal_slot("SymbolData", to_value(random::<i32>()));

    Ok(Gc::new(ValueData::Symbol(GcCell::new(sym_instance))))
}

/// <https://tc39.es/ecma262/#sec-symbol.prototype.tostring>
pub fn to_string(this: &Value, _: &[Value], _: &mut Interpreter) -> ResultValue {
    let s: Value = this.get_internal_slot("Description");
    let full_string = format!("Symbol(\"{}\")", s.to_string());
    Ok(to_value(full_string))
}

/// <https://tc39.es/ecma262/#sec-symbol-constructor>
pub fn create_constructor(global: &Value) -> Value {
    // Create Symbol constructor (or function in Symbol's case)
    let mut symbol_constructor = Object::default();
    symbol_constructor.set_internal_method("call", call_symbol);

    // Create prototype
    let mut symbol_prototype = Object::create(global.clone());

    // Symbol.prototype[[Prototype]] points to Object.prototype
    // Symbol Constructor -> Symbol Prototype -> Object Prototype
    let object_prototype = global.get_field_slice("Object").get_field_slice(PROTOTYPE);
    symbol_prototype.set_internal_slot("Prototype", object_prototype.clone());
    symbol_prototype.set_internal_method("toString", to_string);

    let symbol_prototype_val = to_value(symbol_prototype);
    symbol_constructor.set_internal_slot("Prototype", symbol_prototype_val.clone());

    let symbol_constructor_value = to_value(symbol_constructor);
    symbol_prototype_val.set_field_slice("construcotor", symbol_constructor_value.clone());

    symbol_constructor_value
}
