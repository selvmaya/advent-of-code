use std::collections::{HashMap, HashSet};
#[allow(unused_imports)]
use winnow::{
    ascii::*,
    combinator::*,
    error::*,
    prelude::*,
    stream::*,
    token::*,
    {PResult, Parser},
};

/// This assumes acyclical graph (no cycles, always terminates).
/// Depth-first search, should work if there are no cycles.
/// It should be noted that it would be possible to a partially cyclical graph with this approach
/// if we used early-escaping logical operations to our benefit (see `&&` and `||`).
pub fn descend_graph_cached<'s>(
    target: &'s WireName,
    connections: &'s Connections<'s>,
    known: &mut States<'s>,
) -> bool {
    if known.contains_key(target) {
        *known.get(target).unwrap()
    } else {
        let connection = connections
            .iter()
            .find(|c| &c.dest == target)
            .expect("if not in states, it *has* to come from connections");
        let a = descend_graph_cached(&connection.a, connections, known);
        let b = descend_graph_cached(&connection.b, connections, known);
        let res = match connection.gate {
            Gate::And => a && b,
            Gate::Or => a || b,
            Gate::Xor => a ^ b,
        };
        known.insert(*target, res);
        res
    }
}

pub fn parse_device<'s>(input: &mut &'s str) -> PResult<(States<'s>, Connections<'s>)> {
    separated_pair(
        parse_first_section,
        (line_ending, line_ending),
        parse_second_section,
    )
    .parse_next(input)
}

fn parse_first_section<'s>(input: &mut &'s str) -> PResult<States<'s>> {
    // at least 2 states needed in order to have a working binary gate?
    separated(2.., parse_initial_state, line_ending).parse_next(input)
}

pub type States<'s> = HashMap<WireName<'s>, bool>;

fn parse_initial_state<'s>(input: &mut &'s str) -> PResult<(WireName<'s>, bool)> {
    separated_pair(
        parse_wirename,
        ": ",
        alt(('1'.value(true), '0'.value(false))),
    )
    .parse_next(input)
}

fn parse_second_section<'s>(input: &mut &'s str) -> PResult<Connections<'s>> {
    separated(1.., parse_connection, line_ending).parse_next(input)
}

pub type Connections<'s> = HashSet<Connection<'s>>;

fn parse_connection<'s>(input: &mut &'s str) -> PResult<Connection<'s>> {
    separated_pair(
        parse_wirename,
        ' ',
        separated_pair(
            parse_gate,
            ' ',
            separated_pair(parse_wirename, " -> ", parse_wirename),
        ),
    )
    .map(|(a, (gate, (b, dest)))| Connection { a, b, gate, dest })
    .parse_next(input)
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Connection<'s> {
    pub a: WireName<'s>,
    pub b: WireName<'s>,
    pub gate: Gate,
    pub dest: WireName<'s>,
}

fn parse_wirename<'s>(input: &mut &'s str) -> PResult<WireName<'s>> {
    take(3usize).map(WireName).parse_next(input)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct WireName<'s>(pub &'s str);

fn parse_gate(input: &mut &str) -> PResult<Gate> {
    alt((
        "AND".value(Gate::And),
        "OR".value(Gate::Or),
        "XOR".value(Gate::Xor),
    ))
    .parse_next(input)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Gate {
    And,
    Or,
    Xor,
}
