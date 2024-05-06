use std::borrow::Borrow;

struct CreditCard;
struct WeChatPay;

trait Payment {
    fn pay(&self);
}

impl Payment for CreditCard {
    fn pay(&self) {
        println!("CreditCard pay");
    }
}

impl Payment for WeChatPay {
    fn pay(&self) {
        println!("WeChatPay pay");
    }
}

struct Order {
    payment: Box<dyn Payment>,
}

impl Order {
    fn pay(&self) {
        self.payment.pay();
    }
}

fn run() {
    let order = Order {
        payment: Box::new(CreditCard),
    };

    order.pay();
}
