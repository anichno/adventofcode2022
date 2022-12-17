use std::collections::HashMap;
use std::collections::HashSet;

use petgraph::graph::NodeIndex;
use petgraph::Graph;

#[derive(Debug)]
struct Network {
    graph: Graph<usize, i32, petgraph::Undirected>,
    start: NodeIndex,
}

fn parse(input: &[&str]) -> Network {
    let mut graph = Graph::new_undirected();
    let mut start = None;
    let mut valve_idxs = HashMap::new();
    for line in input {
        let (_, line) = line.split_once(' ').unwrap();
        let (valve, line) = line.split_once(' ').unwrap();
        let (_, line) = line.split_once('=').unwrap();
        let (rate, line) = line.split_once(';').unwrap();
        let (_, line) = line.split_once("valve").unwrap();
        let (_, valves) = line.split_once(' ').unwrap();
        let valves = valves.split(", ");
        let rate: usize = rate.parse().unwrap();
        let node = if let Some(idx) = valve_idxs.get(valve) {
            *graph.node_weight_mut(*idx).unwrap() = rate;
            *idx
        } else {
            let node = graph.add_node(rate);
            valve_idxs.insert(valve, node);
            node
        };

        if valve == "AA" {
            start = Some(node);
        }

        for valve in valves {
            let tgt_idx = if let Some(idx) = valve_idxs.get(valve) {
                *idx
            } else {
                let node = graph.add_node(0);
                valve_idxs.insert(valve, node);
                node
            };

            graph.add_edge(node, tgt_idx, 1);
        }
    }

    Network {
        graph,
        start: start.unwrap(),
    }
}

fn solve1(input: &[&str]) -> usize {
    fn solve_it(
        network: &Network,
        distances: &HashMap<NodeIndex, HashMap<NodeIndex, usize>>,
        cur_pos: NodeIndex,
        time_rem: usize,
        cur_flow: usize,
        unopened: &mut HashSet<NodeIndex>,
        best_flow: &mut usize,
    ) {
        // check if best case turn on all valves beats best
        let mut possible_best = cur_flow;
        for tgt in unopened.iter() {
            possible_best += network.graph[*tgt]
                * (time_rem.saturating_sub(*distances.get(&cur_pos).unwrap().get(tgt).unwrap()));
        }
        if possible_best < *best_flow {
            return;
        }

        if time_rem == 0 {
            *best_flow = (*best_flow).max(cur_flow);

            return;
        }

        // Recurse with having opened cur_pos if we haven't opened it yet
        if unopened.remove(&cur_pos) {
            let new_time_rem = time_rem - 1;
            let new_flow = cur_flow + network.graph[cur_pos] * new_time_rem;

            solve_it(
                network,
                distances,
                cur_pos,
                new_time_rem,
                new_flow,
                unopened,
                best_flow,
            );

            // undo opened
            unopened.insert(cur_pos);
        }

        // Recurse withOUT having opened cur_pos
        for tgt in distances.keys() {
            if *tgt == cur_pos || !unopened.contains(tgt) {
                continue;
            }

            let dist = *distances.get(&cur_pos).unwrap().get(tgt).unwrap();
            if dist < time_rem {
                solve_it(
                    network,
                    distances,
                    *tgt,
                    time_rem - dist,
                    cur_flow,
                    unopened,
                    best_flow,
                );
            }
        }

        solve_it(
            network, distances, cur_pos, 0, cur_flow, unopened, best_flow,
        );
    }

    let network = parse(input);
    let mut distance_map = HashMap::new();
    for node in network.graph.node_indices() {
        if node == network.start || network.graph[node] > 0 {
            let distances: HashMap<NodeIndex, usize> =
                petgraph::algo::dijkstra::dijkstra(&network.graph, node, None, |_| 1);
            distance_map.insert(node, distances);
        }
    }

    let mut best_flow = 0;
    let mut unopened = HashSet::new();
    for node in network.graph.node_indices() {
        if network.graph[node] != 0 {
            unopened.insert(node);
        }
    }
    solve_it(
        &network,
        &distance_map,
        network.start,
        30,
        0,
        &mut unopened,
        &mut best_flow,
    );

    best_flow
}

fn solve2(input: &[&str]) -> usize {
    #[derive(Clone, Copy)]
    struct Order {
        tgt: NodeIndex,
        eta: usize,
    }
    fn solve_it(
        network: &Network,
        distances: &HashMap<NodeIndex, HashMap<NodeIndex, usize>>,
        active_order: &mut Vec<Order>,
        cur_flow: usize,
        unopened: &mut HashSet<NodeIndex>,
        best_flow: &mut usize,
    ) {
        active_order.sort_unstable_by_key(|a| a.eta);
        let Some(active) = active_order.pop() else {
            *best_flow = (*best_flow).max(cur_flow);

            return;

        };

        // check if best case turn on all valves beats best
        let mut possible_best = cur_flow;
        if !active_order.is_empty() {
            possible_best += network.graph[active_order[0].tgt] * active_order[0].eta;
        }

        for tgt in unopened.iter() {
            possible_best += network.graph[*tgt]
                * (active
                    .eta
                    .saturating_sub(*distances.get(&active.tgt).unwrap().get(tgt).unwrap()));
        }
        if possible_best < *best_flow {
            return;
        }

        // Recurse with having opened cur_pos if we haven't opened it yet
        if unopened.remove(&active.tgt) {
            let new_time_rem = active.eta - 1;
            let new_flow = cur_flow + network.graph[active.tgt] * new_time_rem;

            let mut new_orders = active_order.clone();

            new_orders.push(Order {
                tgt: active.tgt,
                eta: new_time_rem,
            });

            solve_it(
                network,
                distances,
                &mut new_orders,
                new_flow,
                unopened,
                best_flow,
            );

            // undo opened
            unopened.insert(active.tgt);
        } else {
            for tgt in distances.keys() {
                if *tgt == active.tgt
                    || !unopened.contains(tgt)
                    || active_order.iter().any(|a| a.tgt == *tgt)
                {
                    continue;
                }

                let dist = *distances.get(&active.tgt).unwrap().get(tgt).unwrap();
                if dist < active.eta {
                    let mut new_orders = active_order.clone();
                    new_orders.push(Order {
                        tgt: *tgt,
                        eta: active.eta - dist,
                    });
                    solve_it(
                        network,
                        distances,
                        &mut new_orders,
                        cur_flow,
                        unopened,
                        best_flow,
                    );
                }
            }
        }

        solve_it(
            network,
            distances,
            active_order,
            cur_flow,
            unopened,
            best_flow,
        );
    }

    let network = parse(input);
    let mut distance_map = HashMap::new();
    for node in network.graph.node_indices() {
        if node == network.start || network.graph[node] > 0 {
            let distances: HashMap<NodeIndex, usize> =
                petgraph::algo::dijkstra::dijkstra(&network.graph, node, None, |_| 1);
            distance_map.insert(node, distances);
        }
    }

    let mut best_flow = 0;
    let mut unopened = HashSet::new();
    for node in network.graph.node_indices() {
        if network.graph[node] != 0 {
            unopened.insert(node);
        }
    }
    let mut active_orders = vec![
        Order {
            tgt: network.start,
            eta: 26,
        },
        Order {
            tgt: network.start,
            eta: 26,
        },
    ];
    solve_it(
        &network,
        &distance_map,
        &mut active_orders,
        0,
        &mut unopened,
        &mut best_flow,
    );

    best_flow
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
        "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
        "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
        "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
        "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
        "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
        "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
        "Valve HH has flow rate=22; tunnel leads to valve GG",
        "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
        "Valve JJ has flow rate=21; tunnel leads to valve II",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 1651)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 1707)
    }
}
