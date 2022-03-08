// run-rustfix
#![warn(clippy::nop_match)]
#![allow(clippy::manual_map)]
#![allow(dead_code)]

enum Choice {
    A,
    B,
    C,
    D,
}

fn useless_match(x: i32) {
    let _: i32 = match x {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => x,
    };
}

fn custom_type_match(se: Choice) {
    let _: Choice = match se {
        Choice::A => Choice::A,
        Choice::B => Choice::B,
        Choice::C => Choice::C,
        Choice::D => Choice::D,
    };
    // Don't trigger
    let _: Choice = match se {
        Choice::A => Choice::A,
        Choice::B => Choice::B,
        _ => Choice::C,
    };
    // Mingled, don't trigger
    let _: Choice = match se {
        Choice::A => Choice::B,
        Choice::B => Choice::C,
        Choice::C => Choice::D,
        Choice::D => Choice::A,
    };
}

fn option_match(x: Option<i32>) {
    let _: Option<i32> = match x {
        Some(a) => Some(a),
        None => None,
    };
    // Don't trigger, this is the case for manual_map_option
    let _: Option<i32> = match x {
        Some(a) => Some(-a),
        None => None,
    };
}

fn func_ret_err<T>(err: T) -> Result<i32, T> {
    Err(err)
}

fn result_match() {
    let _: Result<i32, i32> = match Ok(1) {
        Ok(a) => Ok(a),
        Err(err) => Err(err),
    };
    let _: Result<i32, i32> = match func_ret_err(0_i32) {
        Err(err) => Err(err),
        Ok(a) => Ok(a),
    };
}

fn if_let_option() -> Option<i32> {
    if let Some(a) = Some(1) { Some(a) } else { None }
}

fn if_let_result(x: Result<(), i32>) {
    let _: Result<(), i32> = if let Err(e) = x { Err(e) } else { x };
    let _: Result<(), i32> = if let Ok(val) = x { Ok(val) } else { x };
    // Input type mismatch, don't trigger
    let _: Result<(), i32> = if let Err(e) = Ok(1) { Err(e) } else { x };
}

fn custom_enum_a(x: Choice) -> Choice {
    if let Choice::A = x {
        Choice::A
    } else if let Choice::B = x {
        Choice::B
    } else if let Choice::C = x {
        Choice::C
    } else {
        x
    }
}

fn main() {}
