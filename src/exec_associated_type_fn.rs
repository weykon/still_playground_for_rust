use std::{fmt::format, rc::Rc};

struct Controller {
    name: String,
    wifi: Option<Rc<Wifi>>,
}
trait EmitSignal {
    fn emit(&self, msg: String);
}

impl EmitSignal for Controller {
    fn emit(&self, msg: String) {
        println!("emit signal from {}: {}", self.name, &msg);
    }
}
impl Recevier for Controller {
    fn receive(&self, msg: String) {
        println!("receive signal from {}: {}", self.name, &msg);
    }
}

struct TV {
    name: String,
    wifi: Option<Rc<Wifi>>,
}
trait Recevier {
    fn receive(&self, msg: String);
}
impl Recevier for TV {
    fn receive(&self, msg: String) {
        println!("receive signal from {}: {}", self.name, &msg);
    }
}
impl EmitSignal for TV {
    fn emit(&self, msg: String) {
        println!("emit signal from {}: {}", self.name, &msg);
    }
}

struct Signal<'a> {
    controller: &'a Controller,
    tv: &'a TV,
}
struct WifiSignal<'a> {
    controller: &'a Controller,
    tv: &'a TV,
    ssid: String,             // WiFi网络的SSID
    password: Option<String>, // WiFi网络的密码，可能是None如果网络是开放的
    signal_strength: i32,     // WiFi信号强度
    channel: u8,              // WiFi信号的频道
    encryption_type: String,  // WiFi网络的加密类型
}

struct Wifi {
    ssid: String,             // WiFi网络的SSID
    password: Option<String>, // WiFi网络的密码，可能是None如果网络是开放的
    signal_strength: i32,     // WiFi信号强度
    channel: u8,              // WiFi信号的频道
    encryption_type: String,  // WiFi网络的加密类型
}
impl Wifi {
    fn new(ssid: String, password: Option<String>) -> Wifi {
        Wifi {
            ssid,
            password,
            signal_strength: 100,
            channel: 1,
            encryption_type: "WPA2".to_string(),
        }
    }
}
trait SignalEvent<'a> {
    type To: Recevier + 'a;
    type From: EmitSignal + 'a;
    fn signal_to(to: &'a Self::To, msg: String);
    fn signal_from(from: &'a Self::From, msg: String);
}

impl<'a> SignalEvent<'a> for Signal<'a> {
    type To = TV;
    type From = Controller;
    fn signal_to(to: &'a Self::To, msg: String) {
        to.receive(msg);
    }
    fn signal_from(from: &'a Self::From, msg: String) {
        from.emit(msg);
    }
}
impl<'a> SignalEvent<'a> for WifiSignal<'a> {
    type To = TV;
    type From = Controller;
    fn signal_to(to: &'a Self::To, msg: String) {
        to.receive(msg);
    }
    fn signal_from(from: &'a Self::From, msg: String) {
        let signal_strength = from.wifi.as_ref().unwrap().signal_strength;
        println!("wifi signal strength: {}", signal_strength);
        from.emit(msg);
    }
}
pub fn run() {
    let mut controller = Controller {
        name: "xbox controller".to_string(),
        wifi: None,
    };
    let mut tv = TV {
        name: "canom tv".to_string(),
        wifi: None,
    };
    let wifi = Rc::new(Wifi::new("wifi".to_string(), None));

    controller.wifi = Some(Rc::clone(&wifi));
    tv.wifi = Some(Rc::clone(&wifi));

    let signal = Signal {
        controller: &controller,
        tv: &tv,
    };

    Signal::signal_from(signal.controller, "hello".to_string());
    Signal::signal_to(signal.tv, "hello".to_string());

    let wifi_signal = WifiSignal {
        controller: &controller,
        tv: &tv,
        ssid: wifi.ssid.clone(),
        password: wifi.password.clone(),
        signal_strength: wifi.signal_strength,
        channel: wifi.channel,
        encryption_type: wifi.encryption_type.clone(),
    };
    WifiSignal::signal_from(wifi_signal.controller, "hello by wifi".to_string());
    WifiSignal::signal_to(wifi_signal.tv, "hello by wifi".to_string());

    print_device_name1(&tv);
    print_device_name2(&tv);
    print_device_name1(&controller);
    print_device_name2(&controller);

    print_relfect_msg(&tv);
}

// continue at trait constraint
trait Device: EmitSignal + Recevier {
    fn get_name(&self) -> String;
}
impl Device for TV {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
impl Device for Controller {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
fn print_device_name1(device: &dyn Device) {
    println!("1 device name: {}", device.get_name());
}
fn print_device_name2<T: Device>(device: &T) {
    println!("2 device name: {}", device.get_name());
}

trait ReflectMsg : Recevier { 
    fn reflect_msg(&self, msg: String);
}
impl ReflectMsg for TV { 
    fn reflect_msg(&self, msg: String) { 
        println!("reflect msg from {}: {}", self.name, &msg);
    }
}
fn print_relfect_msg(device: &dyn ReflectMsg) { 
    device.reflect_msg("reflect msg".to_string());
    device.receive(format!("reflect receive print msg again!"));
}