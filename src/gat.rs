use std::fmt::Debug;

trait PointerFamily {
    type PointerType<T>;
}
struct RcPointer;

impl PointerFamily for RcPointer {
    type PointerType<T> = std::rc::Rc<T>;
}

struct ArcPointer;

impl PointerFamily for ArcPointer {
    type PointerType<T> = std::sync::Arc<T>;
}

struct MySystemAtPointerType<T, PointerSelect: PointerFamily> {
    data: PointerSelect::PointerType<T>,
}

trait Character {
    type Weapon: Debug;
    fn weapon(&self) -> Self::Weapon;
    fn name(&self) -> &'static str;
}

struct Warrior;
impl Character for Warrior {
    type Weapon = Sword;
    fn name(&self) -> &'static str {
        "Warrior"
    }
    fn weapon(&self) -> Self::Weapon {
        Sword
    }
}
#[derive(Debug)]
struct Sword;

struct Mage;
impl Character for Mage {
    type Weapon = Staff;
    fn name(&self) -> &'static str {
        "Mage"
    }
    fn weapon(&self) -> Self::Weapon {
        Staff
    }
}
#[derive(Debug)]
struct Staff;

trait Weapon {
    fn name(&self) -> &'static str;
    fn damage(&self) -> u32;
}
impl Weapon for Staff {
    fn name(&self) -> &'static str {
        "Staff"
    }
    fn damage(&self) -> u32 {
        10
    }
}
impl Weapon for Sword {
    fn name(&self) -> &'static str {
        "Sword"
    }
    fn damage(&self) -> u32 {
        20
    }
}
fn attack<T: Character>(character: &T) {
    println!(
        "{:?} Attack with {:?}",
        character.name(),
        character.weapon()
    );
}

trait Factory {
    type Product;
    fn create(&self) -> Self::Product;
}
struct Car;
impl Factory for Car {
    type Product = Car;
    fn create(&self) -> Self::Product {
        Car
    }
}

// 应该保持一个平起平坐的关联层级，而不是一个层级高于另一个层级

trait TakePhone {
    type Phone: Exec;
    type Hand: Exec;
    fn takeup_phone(
        &self,
    ) -> (
        <Self::Phone as Exec>::ExecOutput,
        <Self::Hand as Exec>::ExecOutput,
    );
}
trait Exec {
    type ExecOutput;
    fn exec(&self) -> Self::ExecOutput;
}
impl Exec for Iphone {
    type ExecOutput = Iphone;
    fn exec(&self) -> Iphone {
        println!("is tookup: {}", &self.tookup);
        Iphone { tookup: true }
    }
}
impl Exec for MyHand {
    type ExecOutput = MyHand;
    fn exec(&self) -> MyHand {
        println!("is tookup: {}", &self.tookup);
        MyHand { tookup: true }
    }
}
struct Iphone {
    tookup: bool,
}
struct Me;
struct MyDog;
struct MyHand {
    tookup: bool,
}

struct DogPaw {
    tookup: bool,
}

struct Android {
    tookup: bool,
}
impl Exec for Android {
    type ExecOutput = Android;
    fn exec(&self) -> Android {
        println!("is tookup: {}", &self.tookup);
        Android { tookup: true }
    }
}
impl TakePhone for Me {
    type Phone = Iphone;
    type Hand = MyHand;
    fn takeup_phone(&self) -> (Iphone, MyHand) {
        let phone = Iphone { tookup: true };
        let hand = MyHand { tookup: true };
        println!("me takeup phone");
        (phone.exec(), hand.exec())
    }
}
impl Exec for DogPaw {
    type ExecOutput = DogPaw;
    fn exec(&self) -> DogPaw {
        println!("is tookup: {}", &self.tookup);
        DogPaw { tookup: true }
    }
}
impl TakePhone for MyDog {
    type Phone = Android;
    type Hand = DogPaw;
    fn takeup_phone(&self) -> (Android, DogPaw) {
        let phone = Android { tookup: false };
        let hand = DogPaw { tookup: false };
        println!("dog takeup phone but not ok");
        (phone.exec(), hand.exec())
    }
}

pub fn run() {
    println!("GAT.rs");
    let warrior = Warrior;
    let mage = Mage;
    attack(&warrior);
    attack(&mage);

    let me = Me;
    let dog = MyDog;
    let (phone, hand) = me.takeup_phone();
    let (phone, hand) = dog.takeup_phone(); 
}
