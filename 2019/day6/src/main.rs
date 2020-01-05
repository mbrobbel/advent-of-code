use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io::Read,
    iter::FromIterator,
};

fn count(graph: &HashMap<String, Vec<String>>, node: &str) -> u64 {
    graph.get(node).map_or(0, |values| {
        values.iter().fold(1, |mut acc, x| {
            acc += count(graph, x);
            acc
        })
    })
}

fn get_graph(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|line| {
            let sep = line.find(')').unwrap();
            (line[0..sep].to_string(), line[sep + 1..].to_string())
        })
        .fold(
            HashMap::new(),
            |mut map: HashMap<String, Vec<String>>, (a, b)| {
                map.entry(b)
                    .and_modify(|v| v.push(a.clone()))
                    .or_insert_with(|| vec![a]);
                map
            },
        )
}

fn part_one(input: &str) -> u64 {
    let graph = get_graph(input);
    graph.keys().fold(0, |mut acc, x| {
        acc += count(&graph, x);
        acc
    })
}

fn path_to_center(graph: &HashMap<String, Vec<String>>, node: &str) -> Vec<String> {
    std::iter::successors(Some(node.to_owned()), |x| {
        if let Some(y) = graph.get(x) {
            y.get(0).cloned()
        } else {
            None
        }
    })
    .collect()
}

fn part_two(input: &str) -> usize {
    let graph = get_graph(input);

    let you: HashSet<String> = HashSet::from_iter(path_to_center(&graph, "YOU").into_iter());
    let san = HashSet::from_iter(path_to_center(&graph, "SAN").into_iter());

    you.difference(&san).count() + san.difference(&you).count() - 2
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    println!("part_one: {}", part_one(&input));
    println!("part_two: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_examples() {
        assert_eq!(
            part_one(
                r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"#
            ),
            42
        )
    }

    #[test]
    fn part_two_examples() {
        assert_eq!(
            part_two(
                r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"#
            ),
            4
        )
    }
}
