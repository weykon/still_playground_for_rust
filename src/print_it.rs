trait Printable {
    fn format(&self) -> String;
}

struct SunmiBusiness {
    name: String,
    address: String,
    phone: String,
}

impl Printable for SunmiBusiness {
    fn format(&self) -> String {
        format!(
            "here from Sunmi! \n{}: {} - {}",
            self.name, self.address, self.phone
        )
    }
}

struct GloryBusiness {
    name: String,
    address: String,
    phone: String,
}

impl Printable for GloryBusiness {
    fn format(&self) -> String {
        format!(
            "here from Glory! \n{}: {} - {}",
            self.name, self.address, self.phone
        )
    }
}

struct FromBusiness<T>(T);
impl<T: Printable> FromBusiness<T> {
    fn print(&self) {
        println!("{}", self.0.format());
    }
}
struct PrintService<T: Printable> {
    printer: FromBusiness<T>,
}

impl<T: Printable> PrintService<T> {
    fn print(&self) {
        self.printer.print();
    }
}

fn run() {
    let glory = GloryBusiness {
        name: "Glory".to_string(),
        address: "192.168".to_string(),
        phone: "none".to_string(),
    };

    let print_service = PrintService {
        printer: FromBusiness(glory)
    };

    print_service.print();
}
