use std::fmt::{Debug, Error};

/// Here is the main algorithm that the user is going to want to use.
///
/// Internally, the algorithm uses many "strategies" (I've made it just 3 in
/// this example for brevity).
///
/// I want it to work out of the box (hence the default generics), but also want
/// the user to be able to swap out any/all of them as they desire.
#[derive(Debug, Default)]
struct Algorithm<A = A1, B = B1, C = C1> {
    a: A,
    b: B,
    c: C,
    // potentially more...
}

impl<A, B, C> Algorithm<A, B, C>
where
    A: StrategyA + Debug,
    B: StrategyB + Debug,
    C: StrategyC + Debug,
{
    fn do_something(&self) {
        println!("{self:?}");
        self.a.a();
        self.b.b();
        self.c.c();
    }
}

// define the strategy traits & a couple example implementations for each

trait StrategyA {
    fn a(&self);
}
trait StrategyB {
    fn b(&self);
}
trait StrategyC {
    fn c(&self);
}

#[derive(Debug, Default)]
struct A1;
impl StrategyA for A1 {
    fn a(&self) {
        println!("A1")
    }
}

#[derive(Debug, Default)]
struct A2;
impl StrategyA for A2 {
    fn a(&self) {
        println!("A2")
    }
}

#[derive(Debug, Default)]
struct B1;
impl StrategyB for B1 {
    fn b(&self) {
        println!("B1")
    }
}

#[derive(Debug, Default)]
struct B2;
impl StrategyB for B2 {
    fn b(&self) {
        println!("B2")
    }
}

#[derive(Debug, Default)]
struct C1;
impl StrategyC for C1 {
    fn c(&self) {
        println!("C1")
    }
}

#[derive(Debug, Default)]
struct C2;
impl StrategyC for C2 {
    fn c(&self) {
        println!("C2")
    }
}

/// Try using the builder pattern
struct AlgorithmBuilder<A, B, C> {
    a: Option<A>,
    b: Option<B>,
    c: Option<C>,
}

impl Default for AlgorithmBuilder<A1, B1, C1> {
    fn default() -> Self {
        Self {
            a: None,
            b: None,
            c: None,
        }
    }
}

impl<A, B, C> AlgorithmBuilder<A, B, C> {
    fn a<_A>(self, a: _A) -> AlgorithmBuilder<_A, B, C> {
        AlgorithmBuilder::<_A, B, C> {
            a: Some(a),
            b: self.b,
            c: self.c,
        }
    }

    fn b<_B>(self, b: _B) -> AlgorithmBuilder<A, _B, C> {
        AlgorithmBuilder::<A, _B, C> {
            a: self.a,
            b: Some(b),
            c: self.c,
        }
    }

    fn c<_C>(self, c: _C) -> AlgorithmBuilder<A, B, _C> {
        AlgorithmBuilder::<A, B, _C> {
            a: self.a,
            b: self.b,
            c: Some(c),
        }
    }
}

impl<A: StrategyA + Default, B: StrategyB + Default, C: StrategyC + Default>
    AlgorithmBuilder<A, B, C>
{
    fn build(self) -> Algorithm<A, B, C> {
        Algorithm {
            a: self.a.unwrap_or_else(|| A::default()),
            b: self.b.unwrap_or_else(|| B::default()),
            c: self.c.unwrap_or_else(|| C::default()),
        }
    }
}

/// Here are some examples of how the caller might call it.
pub fn run() {
    let direct_1 = <Algorithm>::default();
    direct_1.do_something();

    let direct_2 = Algorithm::<A2>::default();
    direct_2.do_something();

    // I don't want to have to specify A1 & B1 here... I just want to sway out
    // the C strategy
    let direct_3 = Algorithm::<A1, B1, C2>::default();
    direct_3.do_something();

    let builder_1 = AlgorithmBuilder::default().b(B1).build();
    builder_1.do_something();

    // C1 doesn't have to be specified but A1 still does...
    let builder_2 = AlgorithmBuilder::default().b(B2).build();
    builder_2.do_something();
}