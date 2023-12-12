fn bar<T>() {}

fn baz(f: fn()) {
    println!("{}", std::mem::size_of_val(&f))
}

fn quox<F>(f: F)
where
    F: FnOnce(),
{
}

// read as "For any lifetime 'a, F's implemention of 'a
// of a function from a str reference with a lifetime of a
// to another reference with the same lifetime of a.
fn quox_bound<F>(f: F)
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
}

#[test]
fn main() {
    let mut x = bar::<u32>; // function item
    println!("{}", std::mem::size_of_val(&x));
    baz(bar::<u32>); // => function pointer
    baz(bar::<i32>);

    // impl FnOnce for FnMut, for Fn,
    // can think function pointer impls all these
    quox(bar::<i32>); // trait bound

    let f = || ();
    baz(f); // ok
    quox(f);

    let mut z = String::new();
    let f = || {
        // println!("{}",z);
        // z.clear();  // need FnMut
        drop(z);
    };
    // baz(f); // error
    quox(f);

    let x = || 0; // const closure

    quox_bound(|x| x);
}

fn make_fn() -> impl Fn() {
    let z = String::from("hello");
    move || {
        // need to specify `move` to force the closure to take ownership for `z`.
        println!("Inside closure: {}", z);
    }
}

// RFS: unsized rvalues
fn hello(mut f: Box<dyn FnMut()>) {
    f()
}

// const trait impl
// const fn foo<F: ~const FnOnce()>(f: F) {
//     f()
// }
