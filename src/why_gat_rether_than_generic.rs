// trait Container<T> {
//     fn add(&mut self, item: T);
//     fn remove(&mut self) -> Option<T>;
// }

// struct MyContainer<T> {
//     items: Vec<T>,
// }

// impl<T> Container<T> for MyContainer<T> {
//     fn add(&mut self, item: T) {
//         self.items.push(item);
//     }

//     fn remove(&mut self) -> Option<T> {
//         self.items.pop()
//     }
// }

trait Container {
    type Item;
    fn add(&mut self, item: Self::Item);
    fn remove(&mut self) -> Option<Self::Item>;
}

struct MyContainer<T> {
    items: Vec<T>,
}

impl<T> Container for MyContainer<T> {
    type Item = T;

    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn remove(&mut self) -> Option<T> {
        self.items.pop()
    }
}



// 要先理解泛型和关联类型之间的关系
// 而且当取泛型的时候，泛型的类型是未知的，
// 无法使用HKT来把泛型下的一个特例中使用泛型下的具体类型中的属性或者函数，
// 那么使用关联类型是可以的


pub fn run () { 

}