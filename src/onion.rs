#[derive(Debug)]
struct Skin<'a> {
    color: &'a str,
    thickness: i32,
}
#[derive(Debug)]
struct Onion<'a> {
    skins: Vec<Skin<'a>>,
}

pub fn run() {
    let outer = "outer".to_string(); // 外层生命周期开始
    println!("anew Outer: {}", outer);
    {
        println!("anew   Inner: ");
        let inner = "inner".to_string(); // 内层生命周期开始
        println!("drop   Inner: {}", inner);
    } // 内层生命周期结束

    println!("drop Outer: {}", outer);

    println!("anew   Outer: ");
    let outer = "red".to_string(); // 内层生命周期开始
    {
        let skin1 = Skin {
            color: &outer,
            thickness: 1,
        };
        let outer = "blue".to_string();
        {
            let skin2 = Skin {
                color: &outer,
                thickness: 2,
            };
            {
                let outer = "green".to_string();
                let skin3 = Skin {
                    color: &outer,
                    thickness: 3,
                };
                let onion = Onion {
                    skins: vec![skin1, skin2, skin3],
                };
                println!("Onion: {:?}", onion);
            }
        }
    }
}
