use petgraph::{algo::dijkstra, prelude::GraphMap, Undirected};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};
use string_interner::StringInterner;

use crate::common::day;

pub struct Day16 {
    input: &'static str,
}

impl Day16 {
    pub fn new() -> Self {
        Self {
            input: include_str!("inputs/day16.txt"),
        }
    }
}

impl day::Day for Day16 {
    fn run(&mut self) -> day::Result {
        Ok((Some(run_part1(self.input)?.to_string()), None))
    }
}

type Symbol = string_interner::DefaultSymbol;

fn parse_input_line(
    line: &str,
    interner: &mut StringInterner,
) -> Result<(Symbol, u32, Vec<Symbol>), String> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
                .unwrap();
    }

    if let Some(captures) = RE.captures(line) {
        let tunnels = captures[3]
            .split(", ")
            .map(|s| interner.get_or_intern(s))
            .collect();
        Ok((
            interner.get_or_intern(&captures[1]),
            captures[2].parse::<u32>().map_err(|e| e.to_string())?,
            tunnels,
        ))
    } else {
        Err(format!("Input line {} didn't match regex", line))
    }
}

#[test]
fn test_parse_input_line() {
    let line = "Valve AA has flow rate=60; tunnels lead to valves DD, II, BB";
    let mut interner = StringInterner::new();
    let parsed = parse_input_line(line, &mut interner).expect("This should parse");
    assert_eq!(
        parsed,
        (
            interner.get_or_intern("AA"),
            60,
            vec![
                interner.get_or_intern("DD"),
                interner.get_or_intern("II"),
                interner.get_or_intern("BB")
            ]
        )
    );
}

type ValveGraph = GraphMap<Symbol, (), Undirected>;
type ValveDistances = HashMap<Symbol, HashMap<Symbol, u32>>;

fn make_valve_graph(input: &[(Symbol, u32, Vec<Symbol>)]) -> ValveGraph {
    let edges = input
        .iter()
        .flat_map(|(node, _flow, connections)| connections.iter().map(|c| (*node, *c)));

    ValveGraph::from_edges(edges)
}

fn make_valve_flow_map(input: &[(Symbol, u32, Vec<Symbol>)]) -> HashMap<Symbol, u32> {
    input
        .iter()
        .map(|(valve, flow, _)| (*valve, *flow))
        .collect()
}

fn make_valve_distance_map(valves: &HashSet<Symbol>, valve_graph: &ValveGraph) -> ValveDistances {
    valves
        .iter()
        .map(|start_valve| {
            (
                *start_valve,
                dijkstra(valve_graph, *start_valve, None, |_| 1),
            )
        })
        .collect()
}

#[derive(Clone)]
struct System {
    open_valves: HashSet<Symbol>,
    closed_valves: HashSet<Symbol>,
    valve_distances: ValveDistances,
    valve_flows: HashMap<Symbol, u32>,
    time_remaining: u32,
    current_valve: Symbol,
    water_drained: u32,
    flow_rate: u32,
}

impl System {
    fn from_input(input: &str) -> Result<Self, Box<dyn Error>> {
        let mut interner = StringInterner::new();
        let parsed = input
            .lines()
            .map(|line| parse_input_line(line, &mut interner))
            .collect::<Result<Vec<_>, _>>()?;
        let valve_flows = make_valve_flow_map(&parsed);
        let valves = valve_flows.keys().cloned().collect();
        let graph = make_valve_graph(&parsed);
        let valve_distances = make_valve_distance_map(&valves, &graph);
        let start_valve = *valves
            .iter()
            .find(|v| **v == interner.get_or_intern("AA"))
            .ok_or("couldn't find starting valve 'AA'")?;

        Ok(Self {
            open_valves: HashSet::new(),
            closed_valves: valves,
            valve_distances,
            valve_flows,
            time_remaining: 30,
            current_valve: start_valve,
            water_drained: 0,
            flow_rate: 0,
        })
    }

    fn current_possible_steps(&self) -> Vec<(Symbol, u32)> {
        self.closed_valves
            .iter()
            .map(|v| (v, self.valve_flows[v]))
            .filter_map(|(valve, flow)| {
                if flow == 0 {
                    None
                } else {
                    let distance = self.distance_to(valve);
                    if distance < self.time_remaining {
                        // we can get there before time runs out
                        Some((*valve, distance))
                    } else {
                        None
                    }
                }
            })
            .collect()
    }

    fn distance_to(&self, valve: &Symbol) -> u32 {
        self.valve_distances[&self.current_valve][valve]
    }

    fn step(&mut self) -> Vec<u32> {
        if self.time_remaining == 0 {
            self.finish_time();
            return vec![self.water_drained];
        }
        //self.summarise();

        let choices = self.current_possible_steps();
        if choices.is_empty() {
            self.finish_time();
            return vec![self.water_drained];
        }

        choices
            .into_iter()
            .flat_map(|(valve, distance)| {
                let mut new = self.clone();
                new.drain_water_for(distance);
                new.current_valve = valve;
                new.open_valve(&valve);
                new.step()
            })
            .collect()
    }

    fn open_valve(&mut self, valve: &Symbol) {
        self.open_valves.insert(*valve);
        self.closed_valves.remove(valve);
        self.time_remaining -= 1;
        // new valve's flow isn't counted until after this time period is up
        self.water_drained += self.flow_rate;
        self.flow_rate += self.valve_flows[valve];
    }

    fn drain_water_for(&mut self, time: u32) {
        self.time_remaining -= u32::min(time, self.time_remaining);
        self.water_drained += self.flow_rate * time;
    }

    fn finish_time(&mut self) {
        self.drain_water_for(self.time_remaining);
    }
}

fn run_part1(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut system = System::from_input(input)?;
    let paths = system.step();
    paths
        .into_iter()
        .max()
        .ok_or_else(|| "No paths found".into())
}

#[test]
fn test_part1_sample() {
    let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    let result = run_part1(input).expect("Expect a result");
    assert_eq!(result, 1651);
}
