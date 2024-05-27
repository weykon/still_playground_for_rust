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

#[derive(Debug)]
struct RainWeather {
    rainfall_capacity: i32,
}

#[derive(Debug)]
struct SnowWeather {
    snowfall_capacity: i32,
}
impl RainWeather {
    fn new(rainfall_capacity: i32) -> Self {
        println!("Raining weather: {:?}", rainfall_capacity);
        RainWeather { rainfall_capacity }
    }
}

struct AnimationData<T>
where
    T: Character,
{
    character: T,
    name: String,
    animation: String,
}

trait Animation {
    fn play(&self);
}

trait WeatherAffectCharacter: Character {
    type Weather: Debug;
    fn affect(&self, weather: &Self::Weather);
    fn affect_then_animating(&self, weather: &Self::Weather) {
        self.affect(weather);
        self.walk();
    }
}

impl WeatherAffectCharacter for Person {
    type Weather = RainWeather;
    fn affect(&self, weather: &Self::Weather) {
        println!(
            "Person: {:?} is affected by rain weather: {:?}",
            self.name, weather
        );
    }
}


impl WeatherAffectCharacter for Pet {
    type Weather = RainWeather;
    fn affect(&self, weather: &Self::Weather) {
        println!(
            "Pet: {:?} is affected by rain weather: {:?}",
            self.name, weather.rainfall_capacity
        );
    }
}

impl<T: Character> WeatherAffectCharacter for AnimationData<T> {
    type Weather = RainWeather;
    fn affect(&self, weather: &Self::Weather) {
        println!(
            "Character: {:?} is affected by rain weather: {:?}",
            self.character.get_name(),
            weather
        );
    }
    fn affect_then_animating(&self, weather: &Self::Weather) {
        println!(
            "animaiton running now, who: {}, play:  {}ing animation",
            self.name, self.animation
        );
    }
}

impl<T: Character> Character for AnimationData<T> {
    fn walk(&self) {
        println!("Character: {:?} is walking", self.character.get_name());
    }

    fn stand(&self) {
        println!("Character: {:?} is standing", self.character.get_name());
    }

    fn grow(&self) {
        println!("Character: {:?} is growing", self.character.get_name());
    }

    fn get_name(&self) -> String {
        self.character.get_name()
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
    let rain_weather = RainWeather::new(100);
    me.affect(&rain_weather);
    my_pet.affect(&rain_weather);
    let me_anim = AnimationData {
        character: me,
        name: "me".to_string(),
        animation: "walk".to_string(),
    };
    let my_pet_anim = AnimationData {
        character: my_pet,
        name: "my_pet".to_string(),
        animation: "walk".to_string(),
    };
    me_anim.affect(&rain_weather);
    my_pet_anim.affect(&rain_weather);
    me_anim.affect_then_animating(&rain_weather);
    my_pet_anim.affect_then_animating(&rain_weather);
}
