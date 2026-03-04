pub fn add_curry(a: i32) -> impl Fn(i32) -> i32 {
// fn f() -> Box<dyn Fn(i32) -> i32> { ; another way to tell the compiler that we are returning something that implements the Fn trait, but we don't know the exact type of it. This is useful when we want to return a closure that captures some variables from its environment, but we don't want to specify the exact type of the closure.

    move |b| {
        a + b
    }
}
