// what I not sure which type unknown
// and even had wrap some common similar type

// 首先想一个最底下的T
// 便利在于
// 其实就是为了可以分开写HKT的impl而已

// 抽象不是为了幻想而是为了实现

// 底层无泛型的时候，作为第一层
mod no_generic_at_one {
    trait Calculate {
        type UnknownType;
        fn calculate(&self) -> Self::UnknownType;
    }
}
mod one_generic {
    use std::vec;

    trait Calculate {
        type UnknownType<T>;
        fn calculate<T>(&self) -> Self::UnknownType<T>;
    }
    // 包裹的
    struct ArrayCalc; // 这是一个非常具体到位的实施类了
    impl Calculate for ArrayCalc {
        type UnknownType<T> = Vec<T>;
        fn calculate<T>(&self) -> Self::UnknownType<T> {
            Vec::new()
        }
    }
    struct BoxCalc;
    impl Calculate for BoxCalc {
        type UnknownType<T> = Box<i32>;
        fn calculate<T>(&self) -> Self::UnknownType<T> {
            Box::new(0)
        }
    }

    mod scene {
        use super::*;
        // 比较低的执行代码开始

        // 做桥，跨境，关系点执行图

        // 武器和攻击方式
        // 弓箭武器和角色的动作加载
        // 近战时候加载的范围处理

        trait AttackMethod {
            fn attack(&self);
        }
        impl AttackMethod for LongRange {
            fn attack(&self) {
                todo!()
            }
        }
        impl AttackMethod for CloseCombat {
            fn attack(&self) {
                todo!()
            }
        }
        struct LongRange;
        struct CloseCombat;

        trait BridgeForWeaponAndAttackMethod {
            type Weapon;
            type AttackMethod;
            fn exchange(&self);
        } // 是可以实现切换武器攻击方式的一种实施吗？

        struct AK47CloseCombat;
        struct AK47;
        impl BridgeForWeaponAndAttackMethod for AK47CloseCombat {
            type Weapon = AK47;
            type AttackMethod = CloseCombat;
            fn exchange(&self) {}
        }
    }

    mod more {
        // 武器捡起的代码

        trait Character {
            fn attack(&self);
        }
        struct Pet; // 跟在后面的petpet
        struct Player;
        impl Character for Player {
            fn attack(&self) {
                todo!()
            }
        }
        impl Character for Pet {
            fn attack(&self) {
                todo!()
            }
        }
        trait BridgeForWeaponAndPickupAndCharacter {
            type Weapon;
            type PickUp;
            type Character;
            fn pick_up_weapon(&self);
        }
        struct PlayerPickUpAK47;
        struct PetPickUpAK47;
        struct AK47;
        struct Fork;
        impl Weapon for Fork {
            fn attack(&self) {
                todo!()
            }
        }
        impl Weapon for AK47 {
            fn attack(&self) {
                todo!()
            }
        }
        trait Weapon {
            fn attack(&self);
        }
        impl BridgeForWeaponAndPickupAndCharacter for PlayerPickUpAK47 {
            type Weapon = AK47;
            type PickUp = Player;
            type Character = Player;
            fn pick_up_weapon(&self) {
                // 玩家捡起来的执行
                // 角色需要弯腰检查换弹壳
            }
        }
        impl BridgeForWeaponAndPickupAndCharacter for PetPickUpAK47 {
            type Weapon = AK47;
            type PickUp = Pet;
            type Character = Pet;
            fn pick_up_weapon(&self) {
                // pet的时候检查换弹，需要让玩家角色丢弹药给pet
            }
        }
        struct PlayerPickUpFork;
        impl BridgeForWeaponAndPickupAndCharacter for PlayerPickUpFork {
            type Weapon = Fork;
            type PickUp = Player;
            type Character = Player;
            fn pick_up_weapon(&self) {
                // 角色展开了使用叉子的架势动作
            }
        }
        struct PetPickUpFork;
        impl BridgeForWeaponAndPickupAndCharacter for PetPickUpFork {
            type Weapon = Fork;
            type PickUp = Pet;
            type Character = Pet;
            fn pick_up_weapon(&self) {
                // pet站起来，开始使用叉子攻击
            }
        }
    }

    mod and_more {}
}

//

pub fn run() {}
