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

	fn len(&self) -> u32 {
		match *self {
			Nil => 0,
			Cons(data, ref left, ref right) => 1 + left.len() + right.len(),
		}	
	}

	fn depth(&self) -> u32 {
		match *self {
			Nil => 0,
			Cons(data, ref left, ref right) => {
				let (l, r) = (left.depth(), right.depth());
				if l > r {
					l + 1
				}else{
					r + 1
				}
			}
		}
	}
	
	fn find(&self, elem: u32) -> bool {
		match *self {
			Nil => false,
			Cons(data, ref left, ref right) => {
				if data == elem {
					true
				}else{
					left.find(elem) | right.find(elem)
				}
			},
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
	println!("len {}, depth {}", tree.len(), tree.depth()); 
	println!("{}", tree.stringify());
	println!("6{} 12{}", tree.find(6), tree.find(12));    
}
