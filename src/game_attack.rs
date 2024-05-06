trait Attack {
    fn attack(&self) -> String;
}

struct Sword {
    name: String,
    damage: i32,
}

impl Attack for Sword {
    fn attack(&self) -> String {
        format!("{} attacks with {} damage", self.name, self.damage)
    }
}

struct Magic {
    name: String,
    damage: i32,
}

impl Attack for Magic {
    fn attack(&self) -> String {
        format!("{} attacks with {} damage", self.name, self.damage)
    }
}

fn perform_attack<T: Attack>(item: T) {
    println!("{}", item.attack());
}

struct Character {
    carry_weapon: Weapon,
}

impl Weapon {
    fn attack(&self) -> String {
        self.attack_behavior.attack()
    }
}

struct Weapon {
    name: String,
    damage: i32,
    attack_behavior: Box<dyn Attack>,
}

pub fn run() {
    let sword = Sword {
        name: "Excalibur".to_string(),
        damage: 70,
    };
    perform_attack(sword);
}
