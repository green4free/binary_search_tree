use Tree::*;

enum Tree {
	Cons(u32, Box<Tree>, Box<Tree>),
	Nil,
}

impl Tree {
	fn new() -> Tree {
		Nil
	}

	fn add(self, elem: u32) -> Tree {
		match self{
			Nil => Cons(elem, Box::new(Nil), Box::new(Nil)),
			Cons(data, left, right) => {
				if elem < data {
					Cons(data, Box::new(left.add(elem)), right)
				}else{
					Cons(data, left, Box::new(right.add(elem)))
				}
			},
		}
	}
	
	fn stringify(&self) -> String {
		match *self {
			Nil => format!(""),
			Cons(data, ref left, ref right) => format!("{} {} {}", left.stringify(), data, right.stringify()),
		}
	}
}










fn main() {
	let mut tree = Tree::new();
	tree = tree.add(5);
	tree = tree.add(8);
	tree = tree.add(3);
	tree = tree.add(6);
	tree = tree.add(9);
	tree = tree.add(11);
	tree = tree.add(10);
	tree = tree.add(4);
	println!("{}", tree.stringify());    
}
