struct Node<'a> {
    name: &'a str,
    children: Option<Vec<&'a Node<'a>>>,
}

fn main() {
    let input = include_str!("../input.txt").split("\n");
    let mut head: Node = Node {
        name: "",
        children: None,
    };
}
