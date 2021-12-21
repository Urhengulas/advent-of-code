fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(parse)
        .collect::<Vec<_>>();
    dbg!(snailfish(input));
}

fn parse(mut input: &str) -> Tree {
    if !input.starts_with('[') {
        // IT'S A NUMBER!

        Tree::Num(input.parse().expect(input))
    } else {
        // IT'S A PAIR!

        // strip '[' from the front and ']' from the back
        input = input.strip_prefix('[').expect(input);
        input = input.strip_suffix(']').expect(input);

        // find middle-comma
        let mut mid = None;
        let mut open_counter = 0_u32;
        for (idx, c) in input.char_indices() {
            match c {
                ',' if open_counter == 0 => mid = Some(idx),
                '[' => open_counter += 1,
                ']' => open_counter -= 1,
                _ => {}
            }
        }
        let mid = mid.expect(input);

        // split the pair
        let left = &input[..mid];
        let right = &input[mid + 1..];

        // return a pair-node
        Tree::Pair(Box::new(parse(left)), Box::new(parse(right)))
    }
}

/// Returns the magnitude
fn snailfish(input: Vec<Tree>) -> u32 {
    let tree = input
        .into_iter()
        .reduce(|accum, item| {
            let tree = addition(accum, item);
            reduce(tree)
        })
        .unwrap();
    magnitude(&tree)
}

#[derive(Clone, Debug, PartialEq)]
enum Tree {
    Pair(Box<Tree>, Box<Tree>),
    Num(u32),
    Exploded,
}

impl Tree {
    fn is_num(&self) -> bool {
        match self {
            Tree::Num(..) => true,
            _ => false,
        }
    }
}

fn addition(lhs: Tree, rhs: Tree) -> Tree {
    Tree::Pair(Box::new(lhs), Box::new(rhs))
}

fn magnitude(tree: &Tree) -> u32 {
    match &tree {
        Tree::Pair(left, right) => 3 * magnitude(left) + 2 * magnitude(right),
        Tree::Num(num) => *num,
        Tree::Exploded => unreachable!("only present _during_ explosion"),
    }
}

fn reduce(mut tree: Tree) -> Tree {
    loop {
        let tmp_tree = tree.clone();

        tree = explode(tree);
        if tree != tmp_tree {
            continue;
        }

        tree = split(tree);
        if tree != tmp_tree {
            continue;
        }

        break tree;
    }
}

fn explode(mut tree: Tree) -> Tree {
    let mut depth = 0_u32;
    let mut values = None;

    tree = explode_search(tree, &mut depth, &mut values);

    if let Some((left, right)) = values {
        let mut found;

        found = false;
        tree = explode_add(tree, left, &mut found, Direction::RightToLeft);

        found = false;
        tree = explode_add(tree, right, &mut found, Direction::LeftToRight);

        tree = explode_replace(tree);
    }

    tree
}

fn explode_search(mut tree: Tree, depth: &mut u32, values: &mut Option<(u32, u32)>) -> Tree {
    *depth += 1;

    if values.is_none() {
        tree = match tree {
            Tree::Pair(left, right) if *depth > 4 && left.is_num() && right.is_num() => {
                match (*left, *right) {
                    (Tree::Num(left), Tree::Num(right)) => *values = Some((left, right)),
                    _ => unreachable!(),
                }
                Tree::Exploded
            }
            Tree::Pair(left, right) => Tree::Pair(
                Box::new(explode_search(*left, depth, values)),
                Box::new(explode_search(*right, depth, values)),
            ),
            Tree::Exploded => unreachable!("only present _during_ explosion"),
            Tree::Num(_) => tree,
        }
    }

    *depth -= 1;
    tree
}

#[derive(Clone, Copy)]
enum Direction {
    RightToLeft,
    LeftToRight,
}

fn explode_add(tree: Tree, value: u32, found: &mut bool, dir: Direction) -> Tree {
    match tree {
        Tree::Exploded => {
            *found = true;
            tree
        }
        Tree::Pair(mut left, mut right) => {
            match dir {
                Direction::RightToLeft => {
                    right = Box::new(explode_add(*right, value, found, dir));
                    left = Box::new(explode_add(*left, value, found, dir));
                }
                Direction::LeftToRight => {
                    left = Box::new(explode_add(*left, value, found, dir));
                    right = Box::new(explode_add(*right, value, found, dir));
                }
            }
            Tree::Pair(left, right)
        }
        Tree::Num(num) if *found => {
            *found = false;
            Tree::Num(num + value)
        }
        Tree::Num(_) => tree,
    }
}

fn explode_replace(tree: Tree) -> Tree {
    match tree {
        Tree::Exploded => Tree::Num(0),
        Tree::Pair(left, right) => Tree::Pair(
            Box::new(explode_replace(*left)),
            Box::new(explode_replace(*right)),
        ),
        Tree::Num(_) => tree,
    }
}

fn split(tree: Tree) -> Tree {
    let mut activated = false;
    inner_split(tree, &mut activated)
}

fn inner_split(tree: Tree, activated: &mut bool) -> Tree {
    if *activated {
        return tree;
    }

    match tree {
        Tree::Pair(left, right) => Tree::Pair(
            Box::new(inner_split(*left, activated)),
            Box::new(inner_split(*right, activated)),
        ),
        Tree::Num(num) if num >= 10 => {
            *activated = true;
            Tree::Pair(
                Box::new(Tree::Num((num as f64 / 2.0).floor() as u32)),
                Box::new(Tree::Num((num as f64 / 2.0).ceil() as u32)),
            )
        }
        Tree::Num(_) => tree,
        Tree::Exploded => unreachable!("only present _during_ explosion"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explode_1() {
        assert_eq!(
            explode(parse("[[[[[9,8],1],2],3],4]")),
            parse("[[[[0,9],2],3],4]")
        );
    }

    #[test]
    fn explode_2() {
        assert_eq!(
            explode(parse("[7,[6,[5,[4,[3,2]]]]]")),
            parse("[7,[6,[5,[7,0]]]]")
        );
    }

    #[test]
    fn explode_3() {
        assert_eq!(
            explode(parse("[[6,[5,[4,[3,2]]]],1]")),
            parse("[[6,[5,[7,0]]],3]")
        );
    }

    #[test]
    fn explode_4() {
        assert_eq!(
            explode(parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")),
            parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
        );
    }

    #[test]
    fn explode_5() {
        assert_eq!(
            explode(parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")),
            parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
        );
    }

    #[test]
    fn addition_1() {
        assert_eq!(
            addition(parse("[1,2]"), parse("[[3,4],5]")),
            parse("[[1,2],[[3,4],5]]")
        );
    }

    #[test]
    fn addition_2() {
        assert_eq!(
            addition(parse("[[[[4,3],4],4],[7,[[8,4],9]]]"), parse("[1,1]")),
            parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")
        );
    }

    #[test]
    fn reduce_1() {
        assert_eq!(
            reduce(parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")),
            parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }

    #[test]
    fn magnitude_1() {
        assert_eq!(magnitude(&parse("[9,1]")), 29);
    }

    #[test]
    fn magnitude_2() {
        assert_eq!(magnitude(&parse("[1,9]")), 21);
    }

    #[test]
    fn magnitude_3() {
        assert_eq!(magnitude(&parse("[[1,2],[[3,4],5]]")), 143);
    }

    #[test]
    fn magnitude_4() {
        assert_eq!(magnitude(&parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")), 1384);
    }

    #[test]
    fn magnitude_5() {
        assert_eq!(magnitude(&parse("[[[[1,1],[2,2]],[3,3]],[4,4]]")), 445);
    }

    #[test]
    fn magnitude_6() {
        assert_eq!(magnitude(&parse("[[[[3,0],[5,3]],[4,4]],[5,5]]")), 791);
    }

    #[test]
    fn magnitude_7() {
        assert_eq!(magnitude(&parse("[[[[5,0],[7,4]],[5,5]],[6,6]]")), 1137);
    }

    #[test]
    fn magnitude_8() {
        assert_eq!(
            magnitude(&parse(
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            )),
            3488
        );
    }

    #[test]
    fn snailfish_1() {
        assert_eq!(
            snailfish(vec![
                parse("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]"),
                parse("[[[5,[2,8]],4],[5,[[9,9],0]]]"),
                parse("[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]"),
                parse("[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]"),
                parse("[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]"),
                parse("[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]"),
                parse("[[[[5,4],[7,7]],8],[[8,3],8]]"),
                parse("[[9,3],[[9,9],[6,[4,9]]]]"),
                parse("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]"),
                parse("[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"),
            ]),
            4140
        )
    }

    #[test]
    fn split_1() {
        assert_eq!(
            split(parse("[[[[0,7],4],[15,[0,13]]],[1,1]]")),
            parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")
        );
    }

    #[test]
    fn split_2() {
        assert_eq!(
            split(parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")),
            parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]")
        );
    }

    #[test]
    fn reduce_untouched_1() {
        assert_eq!(reduce(parse("[1,[2,3]]")), parse("[1,[2,3]]"));
    }
}
