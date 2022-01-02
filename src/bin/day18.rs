const INPUT_FILE: &str = include_str!("../../inputs/day18.txt");

type ChildNode<T> = Option<Box<BTNode<T>>>;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op<T> {
    Pair,
    Id(T),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BTNode<T> {
    left: ChildNode<T>,
    right: ChildNode<T>,
    op: Op<T>,
}

impl BTNode<i64> {
    pub fn new(op: Op<i64>, l: BTNode<i64>, r: BTNode<i64>) -> Self {
        BTNode::<i64> {
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
            op: op,
        }
    }

    pub fn unparse(&self) -> String {
        match self.op {
            Op::Id(value) => value.to_string(),
            Op::Pair => format!(
                "[{},{}]",
                self.left.as_ref().unwrap().unparse(),
                self.right.as_ref().unwrap().unparse()
            ),
        }
    }

    pub fn find_exploding_parent(&mut self, depth: usize) -> Option<&mut BTNode<i64>> {
        if depth == 3 {
            return Some(self);
        }

        if let Some(left) = &mut self.left {
            return left.find_exploding_parent(depth + 1);
        }

        if let Some(right) = &mut self.right {
            return right.find_exploding_parent(depth + 1);
        }
        None
    }

    pub fn explode_descedants(&mut self) {}
}

fn pair_node(l: BTNode<i64>, r: BTNode<i64>) -> BTNode<i64> {
    BTNode::new(Op::Pair, l, r)
}

fn id_node(value: i64) -> BTNode<i64> {
    BTNode {
        left: None,
        right: None,
        op: Op::Id(value),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct BinaryTree<T> {
    head: Option<BTNode<T>>,
}

impl BinaryTree<i64> {
    pub fn new(head: BTNode<i64>) -> Self {
        BinaryTree::<i64> { head: Some(head) }
    }

    pub fn unparse(&self) -> String {
        self.head.as_ref().unwrap().unparse()
    }

    pub fn explode(&mut self) {
        if let Some(mut exploding_parent_node) =
            self.head.as_mut().unwrap().find_exploding_parent(0)
        {
            println!("EXPLODING PARENT FOUND {:?}", exploding_parent_node);

            let descedant_left = exploding_parent_node.left.as_ref().unwrap();
            let descedant_right = exploding_parent_node.right.as_ref().unwrap();

            let mut new_left = id_node(0);
            let mut new_right = id_node(0);
            if let Op::Pair = descedant_left.op {
                if let Op::Id(value) = descedant_left.right.as_ref().unwrap().op {
                    if let Op::Id(value2) = exploding_parent_node.right.as_ref().unwrap().op {
                        println!("value left{:?}", value);
                        new_right = id_node(value + value2);
                    }
                }
            }

            exploding_parent_node.left = Some(Box::new(new_left));
            exploding_parent_node.right = Some(Box::new(new_right));
        }
    }

    pub fn parse(input: String) -> Self {
        let chars = input.trim().chars();

        let mut stack: Vec<_> = vec![];
        for c in chars {
            match c {
                '[' => {
                    stack.push(BTNode {
                        left: None,
                        right: None,
                        op: Op::Pair,
                    });
                }
                ']' => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    let mut node = stack.pop().unwrap();

                    node.right = Some(Box::new(right));
                    node.left = Some(Box::new(left));

                    stack.push(node);
                }
                _ => {
                    if c.is_digit(10) {
                        stack.push(id_node(c.to_digit(10).unwrap() as i64));
                    }
                }
            };
        }
        BinaryTree::new(stack.pop().unwrap())
    }
}

fn part_1(input: String) -> i64 {
    let bt = BinaryTree::parse(input);
    0
}

fn main() {
    let res1 = part_1(INPUT_FILE.to_string());
    println!("part1: {}", res1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_nested() {
        let parsed = BinaryTree::parse("[[1,2],3]".to_string());
        assert_eq!(parsed.unparse(), "[[1,2],3]")
    }

    #[test]
    fn parses_example() {
        let parsed = BinaryTree::parse("[[[[4,3],4],4],[7,[[8,4],9]]]".to_string());
        assert_eq!(parsed.unparse(), "[[[[4,3],4],4],[7,[[8,4],9]]]")
    }

    #[test]
    fn explodes_example() {
        let mut tree = BinaryTree::parse("[[[[[9,8],1],2],3],4]".to_string());
        tree.explode();
        assert_eq!(tree.unparse(), "[[[[0,9],2],3],4]");
    }

    #[test]
    fn solves_part1_example() {
        assert_eq!(part_1("[[1,2],[[3,4],5]]".to_string()), 143);
    }
}
