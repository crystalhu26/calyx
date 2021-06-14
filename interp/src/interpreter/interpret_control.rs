//! Inteprets a control in a component.

use super::interpret_group::interpret_group;
use crate::environment::Environment;
use calyx::{errors::FutilResult, ir};

/// Helper function to evaluate control
pub fn interpret_control(
    ctrl: &ir::Control,
    continuous_assignments: &[ir::Assignment],
    env: Environment,
) -> FutilResult<Environment> {
    match ctrl {
        ir::Control::Seq(s) => eval_seq(s, continuous_assignments, env),
        ir::Control::Par(p) => eval_par(p, continuous_assignments, env),
        ir::Control::If(i) => eval_if(i, continuous_assignments, env),
        ir::Control::While(w) => eval_while(w, continuous_assignments, env),
        ir::Control::Invoke(i) => eval_invoke(i, continuous_assignments, env),
        ir::Control::Enable(e) => eval_enable(e, continuous_assignments, env),
        ir::Control::Empty(e) => eval_empty(e, continuous_assignments, env),
    }
}

/// Interpret Seq
fn eval_seq(
    s: &ir::Seq,
    continuous_assignments: &[ir::Assignment],
    mut env: Environment,
) -> FutilResult<Environment> {
    for stmt in &s.stmts {
        env = interpret_control(stmt, continuous_assignments, env)?;
    }
    Ok(env)
}

/// Interpret Par
/// at the moment behaves like seq
fn eval_par(
    _p: &ir::Par,
    _continuous_assignments: &[ir::Assignment],
    mut _env: Environment,
) -> FutilResult<Environment> {
    // for stmt in &p.stmts {
    //     env = interpret_control(stmt, comp.clone(), env)?;
    // }
    todo!()
}

/// Interpret If
fn eval_if(
    i: &ir::If,
    continuous_assignments: &[ir::Assignment],
    mut env: Environment,
) -> FutilResult<Environment> {
    env =
        interpret_group(&i.cond.borrow(), continuous_assignments, env).unwrap();

    if env.get_from_port(&i.port.borrow()).as_u64() == 0 {
        env =
            interpret_control(&i.fbranch, continuous_assignments, env).unwrap();
        Ok(env)
    } else {
        env =
            interpret_control(&i.tbranch, continuous_assignments, env).unwrap();
        Ok(env)
    }
}

/// Interpret While
// /// The loop statement is similar to the conditional. It enables
// cond_group and uses port_name as the conditional value. When the
// value is high, it executes body_stmt and recomputes the conditional
// using cond_group.
fn eval_while(
    w: &ir::While,
    continuous_assignments: &[ir::Assignment],
    mut env: Environment,
) -> FutilResult<Environment> {
    env =
        interpret_group(&w.cond.borrow(), continuous_assignments, env).unwrap();

    if env.get_from_port(&w.port.borrow()).as_u64() == 1 {
        env = interpret_control(&w.body, continuous_assignments, env).unwrap();
        return eval_while(w, continuous_assignments, env);
    }

    return Ok(env);
    // // currently ports don't update properly in mutli-cycle and runs into infinite loop
    // // count needs to be removed when the infinite loop problem is fixed
    // let mut count = 0;
    // while env.get_from_port(&comp, &w.port.borrow()) != 1 && count < 5 {
    //     env = interpret_control(&w.body, comp, env)?;
    //     env = interpret_group(&w.cond.borrow(), env, comp)?;
    //     // count needs to be remved
    //     count += 1;
    // }
    // Ok(env)
}

/// Interpret Invoke
/// TODO
#[allow(clippy::unnecessary_wraps)]
fn eval_invoke(
    _i: &ir::Invoke,
    _continuous_assignments: &[ir::Assignment],
    _env: Environment,
) -> FutilResult<Environment> {
    todo!()
}

/// Interpret Enable
fn eval_enable(
    e: &ir::Enable,
    continuous_assignments: &[ir::Assignment],
    env: Environment,
) -> FutilResult<Environment> {
    interpret_group(&e.group.borrow(), continuous_assignments, env)
}

/// Interpret Empty
#[allow(clippy::unnecessary_wraps)]
fn eval_empty(
    _e: &ir::Empty,
    _continuous_assignments: &[ir::Assignment],
    env: Environment,
) -> FutilResult<Environment> {
    Ok(env)
}