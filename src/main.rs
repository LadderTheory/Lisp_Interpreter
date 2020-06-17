
struct Item {
    s: String,
}

impl Item {
    pub fn itemize(&mut self) -> Vec<Item> {
        let mut r = vec![];

        let operators = vec!['+','-','/','*'];

        let mut i = 0;
        while i < self.s.len() {
            let oldi = i;
            let c = |x| -> char {
                self.s.clone().chars().nth(x).unwrap()
            };

            if c(i) == '(' {
                let mut child = String::from("");
                let mut openings = 0;
                let mut closings = 0;
                while i < self.s.len() {
                    child.push(c(i));

                    if c(i) == '(' {
                        openings += 1;
                    }

                    if c(i) == ')' {
                        closings += 1;
                        if closings == openings {
                            break;
                        }
                    }
                    i += 1;
                }

                r.push(Item::new(child));
            }

            if operators.contains(&c(i)) {
                r.push(Item::new(c(i).to_string()));
            }

            let mut dig = String::from("");
            while c(i).is_numeric() {
                dig.push(c(i));
                i += 1;
            }
            if dig.len() > 0 {
                r.push(Item::new(dig));
            }
            
            if (oldi == i) {//default case
                i += 1;
            }
        }
        
        r
    }

    pub fn new(arg: String) -> Item {
        Item {
            s: arg,
        }
    }
}

fn main() {
    let test = vec![
        "(2+ (3 - 4))",
        "((3/84)+7)",
    ];

    for i in 0..test.len() {
        let items = Item::new(test[i].to_string()).itemize();
        items.iter().for_each(|x| println!("{}", x.s));
        println!();
    }
}
