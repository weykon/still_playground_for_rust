trait WeaponSightingTelescopeSocket {
    type Weapon;
    type SightingTelescope;
    fn connect(&self, weapon: Self::Weapon, sighting_telescope: Self::SightingTelescope);
}

trait RedPointSightingTelescopeSocket {
    type Weapon;
    type SightingTelescope;
    fn connect(&self, weapon: Self::Weapon, sighting_telescope: Self::SightingTelescope);
}

trait Weapon {
    fn shoot(&self);
    fn change_sighting_telescope<T: SightingTelescope>(&self, sighting_telescope: T);
}

trait SightingTelescope {
    fn to_exchange_happen_first(&self);
}
struct M4;
struct AK47;
struct RedPoint;
struct ACOG;

// 这里如果想实现不仅仅对RedPoint有效，还需要对ACOG有效的话，
// 要么定义一个新的对应例如WeaponSightingTelescopeSocketRedPoint trait
// 或者是 for M4WhenRedPoint 这样

impl RedPointSightingTelescopeSocket for M4 {
    type Weapon = M4;
    type SightingTelescope = RedPoint;
    fn connect(&self, weapon: M4, sighting_telescope: RedPoint) {
        sighting_telescope.to_exchange_happen_first();
        sighting_telescope.point_up();
    }
}

// 不同的 trait：为每种 SightingTelescope 类型定义不同的 trait，实现不同的行为。
// 泛型和特化：使用泛型方法，并在方法内部处理不同的 SightingTelescope 类型。
// 使用 enum：如果 SightingTelescope 类型是固定的，可以使用 enum 来区分不同的类型。

impl Weapon for M4 {
    fn shoot(&self) {
        println!("M4 shoot");
    }
    fn change_sighting_telescope<T: SightingTelescope>(&self, sighting_telescope: T) {
        sighting_telescope.to_exchange_happen_first();
    }
}
impl RedPoint{
    fn point_up(&self){
        println!("RedPoint point_up");
    }
}
impl SightingTelescope for RedPoint {
    fn to_exchange_happen_first(&self) {
        println!("RedPoint to_exchange_happen_first");
    }
}
impl SightingTelescope for ACOG {
    fn to_exchange_happen_first(&self) {
        println!("ACOG to_exchange_happen_first");
    }
}
pub fn run() {
    let m4 = M4;
    let red_point = RedPoint;
    let acog = ACOG;
    m4.change_sighting_telescope(red_point);
    m4.change_sighting_telescope(acog);
    m4.shoot();
}
