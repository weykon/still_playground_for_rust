use std::borrow::Borrow;

trait Hero {
    fn attack_power(&self) -> i32;
    fn hp(&self) -> i32;
    fn take_damage(&mut self, damage: i32);
    fn name(&self) -> String;
}
struct Raz {
    attack_power: i32,
    hp: i32,
}
impl Hero for Raz {
    fn attack_power(&self) -> i32 {
        self.attack_power
    }
    fn hp(&self) -> i32 {
        self.hp
    }
    fn take_damage(&mut self, damage: i32) {
        println!("Raz take damage: {}", damage);
        self.hp = self.hp.saturating_sub(damage);
    }
    fn name(&self) -> String {
        "Raz".to_string()
    }
}
struct EZ {
    attack_power: i32,
    hp: i32,
}

impl Hero for EZ {
    fn attack_power(&self) -> i32 {
        self.attack_power
    }
    fn hp(&self) -> i32 {
        self.hp
    }
    fn take_damage(&mut self, damage: i32) {
        println!("EZ take damage: {}", damage);
        self.hp = self.hp.saturating_sub(damage);
    }
    fn name(&self) -> String {
        "EZ".to_string()
    }
}

trait Fight<T: Hero> {
    fn fight(&self, to: &mut T);
}

impl<T> Fight<T> for EZ
where
    T: Hero,
{
    // EZ 对打击Hero
    fn fight(&self, to: &mut T) {
        println!("EZ 发起对{}的攻击", to.name());
        to.take_damage(self.attack_power);
    }
}
impl<T> Fight<T> for Raz
where
    T: Hero,
{
    fn fight(&self, to: &mut T) {
        println!("Raz 发起对{}的攻击", to.name());
        to.take_damage(self.attack_power);
    }
}

struct GameStart<'a> {
    heroes: Vec<&'a dyn Hero>,
}

pub fn run() {
    let mut game = GameStart { heroes: Vec::new() };

    let mut ez = EZ {
        attack_power: 10,
        hp: 100,
    };
    let mut raz = Raz {
        attack_power: 40,
        hp: 100,
    };

    game.heroes.push(&ez);
    game.heroes.push(&raz);

    ez.fight(&mut raz);
    raz.fight(&mut ez);
}
