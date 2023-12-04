use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, greeting: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("{}", self.greeting),
            VisitorAction::AcceptWithNote { note } => {
                println!("{}", self.greeting);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a friend of the treehouse", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("harin", "Howdy Hao!", VisitorAction::Accept, 24),
        Visitor::new(
            "anna",
            "Hello AnnaBanana!",
            VisitorAction::AcceptWithNote {
                note: String::from("Give free drink!"),
            },
            23,
        ),
        Visitor::new("Harinder", "Who?", VisitorAction::Refuse, 23),
    ];

    loop {
        println!("Hello, what is your name?");
        let name = what_is_your_name();
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(
                        &name,
                        "New friend",
                        VisitorAction::Probation,
                        20,
                    ));
                }
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}
