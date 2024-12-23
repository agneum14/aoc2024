#![allow(dead_code)]
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::get_input;

struct LanParty {
    edges: HashMap<String, Vec<String>>,
}

impl LanParty {
    fn new(input: &str) -> Self {
        let mut edges = HashMap::new();
        for line in input.lines() {
            let (a, b) = line.split_once("-").unwrap();
            for (a, b) in [(a, b), (b, a)] {
                edges
                    .entry(a.to_owned())
                    .or_insert(Vec::new())
                    .push(b.to_owned());
            }
        }
        Self { edges }
    }

    fn password(&self) -> String {
        let mut max_clique = HashSet::new();
        self.bron_kerbosch(
            &mut max_clique,
            HashSet::new(),
            self.edges.keys().cloned().collect::<HashSet<_>>(),
            HashSet::new(),
        );
        max_clique.iter().sorted().join(",")
    }

    fn bron_kerbosch(
        &self,
        max_clique: &mut HashSet<String>,
        r: HashSet<String>,
        p: HashSet<String>,
        x: HashSet<String>,
    ) {
        if p.is_empty() && x.is_empty() {
            if r.len() > max_clique.len() {
                *max_clique = r;
            }
            return;
        }
        let mut p = p;
        let mut x = x;
        let u = p.union(&x).into_iter().next().unwrap();
        for v in p
            .clone()
            .difference(&self.edges[u].iter().cloned().collect::<HashSet<_>>())
        {
            let n = &self.edges[v].iter().cloned().collect::<HashSet<_>>();
            let mut r = r.clone();
            r.insert(v.to_string());
            self.bron_kerbosch(
                max_clique,
                r,
                p.clone().intersection(n).into_iter().cloned().collect(),
                x.clone().intersection(n).into_iter().cloned().collect(),
            );
            p.remove(v);
            x.insert(v.to_string());
        }
    }

    fn t_cliques(&self) -> usize {
        let mut starting = HashSet::new();
        self.edges
            .keys()
            .into_iter()
            .filter(|comp| comp.starts_with("t"))
            .cloned()
            .flat_map(|k| {
                let res =
                    self.interconnected(Vec::from([k.to_string()]), HashSet::new(), &mut starting);
                starting.insert(k.to_string());
                res
            })
            .count()
            / 2
    }

    fn interconnected(
        &self,
        clique: Vec<String>,
        candidates: HashSet<String>,
        starting: &HashSet<String>,
    ) -> Vec<Vec<String>> {
        if clique.len() == 3 {
            return Vec::from([clique]);
        }

        let mut candidates = candidates;
        let neighbors = self.edges[clique.last().unwrap()]
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        if candidates.len() == 0 {
            candidates.extend(neighbors);
        } else {
            candidates = candidates
                .intersection(&neighbors)
                .cloned()
                .collect::<HashSet<_>>();
        }
        candidates
            .clone()
            .into_iter()
            .filter(|c| !starting.contains(c))
            .flat_map(|c| {
                let mut new_clique = clique.clone();
                new_clique.push(c);
                self.interconnected(new_clique, candidates.clone(), starting)
            })
            .collect()
    }
}

pub fn run() {
    let lp = LanParty::new(&get_input(23));
    println!("T cliques of 3: {}", lp.t_cliques());
    println!("Password: {}", lp.password());
}

#[cfg(test)]
mod tests {
    use crate::{day23::LanParty, utils::get_small};

    #[test]
    fn t_cliques() {
        let lp = LanParty::new(&get_small(23));
        assert_eq!(7, lp.t_cliques())
    }
}
