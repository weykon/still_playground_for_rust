use std::fmt::Debug;

trait Character {
    fn walk(&self);
    fn stand(&self);
    fn grow(&self);
    fn get_name(&self) -> String;
}

struct Person {
    name: String,
    model: Option<Box<Vec<String>>>,
    age: i32,
}

struct Pet {
    name: String,
    model: Option<Box<Vec<String>>>,
    age: i32,
}

trait Weather {
    fn get_description(&self) -> String;
}

struct SnowWeather {
    temperature: i32,
}

impl Weather for SnowWeather {
    fn get_description(&self) -> String {
        format!("Snowy with temperature: {}", self.temperature)
    }
}

struct SunnyWeather {
    temperature: i32,
}

impl Weather for SunnyWeather {
    fn get_description(&self) -> String {
        format!("Sunny with temperature: {}", self.temperature)
    }
}

// 以下这样是三个trait在一起的定义。非常神奇。
trait WeatherAffectCharacter<W: Weather>: Character {
    fn affect_by(&self, weather: &W);
    fn affected_then_animating(&self, weather: &W) {
        self.affect_by(weather);
        self.walk();
    }
}

impl<W: Weather> WeatherAffectCharacter<W> for Pet {
    fn affect_by(&self, weather: &W) {
        println!(
            "Pet: {:?} is affected by weather: {:?}",
            self.get_name(),
            weather.get_description()
        );
    }
}

impl<W: Weather> WeatherAffectCharacter<W> for Person {
    fn affect_by(&self, weather: &W) {
        println!(
            "Person: {:?} is affected by weather: {:?}",
            self.get_name(),
            weather.get_description()
        );
    }
}

impl Character for Person {
    fn walk(&self) {
        println!("Person: {:?} is walking", self.name);
    }

    fn stand(&self) {
        println!("Person: {:?} is standing", self.name);
    }

    fn grow(&self) {
        println!("Person: {:?} is growing", self.name);
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Character for Pet {
    fn walk(&self) {
        println!("Pet: {:?} is walking", self.name);
    }

    fn stand(&self) {
        println!("Pet: {:?} is standing", self.name);
    }

    fn grow(&self) {
        println!("Pet: {:?} is growing", self.name);
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub fn run() {
    println!("Still at GAT");
    let me = Person {
        name: "me".to_string(),
        model: None,
        age: 18,
    };
    let my_pet = Pet {
        name: "my_pet".to_string(),
        model: None,
        age: 2,
    };
    let snow_weather = SnowWeather { temperature: -5 };
    let sunny_weather = SunnyWeather { temperature: 25 };

    me.affect_by(&snow_weather);
    my_pet.affect_by(&sunny_weather);

    me.affected_then_animating(&snow_weather);
    my_pet.affected_then_animating(&sunny_weather);
}
