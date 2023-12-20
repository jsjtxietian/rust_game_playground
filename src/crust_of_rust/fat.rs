use std::iter::Extend;
use std::vec;

// dyn Trair -> * -> (*mut data, *mut vtable)
// [u8]      -> * -> (*mut data, usize length)
// str       -> * -> (*mut data, usize length)

// take a look at 2580-ptr-meta, the RawWaker
fn t1(s: &[u8]) {}

fn t2() -> Box<[u8]> {
    Box::new([]) as Box<[u8]>
}

pub fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

pub fn strlen2<S>(s: S) -> usize
where
    S: AsRef<str>,
{
    s.as_ref().len()
}

pub fn strlen3<S: AsRef<str>>(s: S) -> usize {
    s.as_ref().len()
}

pub fn strlen_dyn(s: Box<dyn AsRef<str>>) -> usize {
    s.as_ref().as_ref().len()
}

pub fn strlen_dyn2(s: &dyn AsRef<str>) -> usize {
    s.as_ref().len()
}

pub fn main() {
    let x: Box<dyn AsRef<str>> = Box::new(String::from("hello"));
    strlen_dyn(x);

    let y: &dyn AsRef<str> = &"world";
    strlen_dyn2(y);
}

pub trait Hei {
    // type Name;
    fn hei(&self) {}

    // can't be called from trait object, not in vtable
    fn weird(&self)
    where
        Self: Sized,
    {
    }
}

impl Hei for &str {
    // type Name = ();

    fn hei(&self) {
        print!("Hei {}", self)
    }
}

impl Hei for String {
    // type Name = ();

    fn hei(&self) {
        print!("Hei {}", self)
    }
}

pub fn say_hei_static<H: Hei>(s: H) {
    s.hei();
}

pub fn say_hei(s: &dyn Hei) {
    s.hei();
    // s.weird();
    // (dyn Hei)::weird();
}

pub trait HeiAsRef: Hei + AsRef<str> {}

// pub fn baz(s: &(dyn Hei + AsRef<str>)){
pub fn baz(s: &dyn HeiAsRef) {
    s.hei();
    let s = s.as_ref();
    let _ = s.len();
}

// pub fn foo() {
//     "J".hei()
// }

// pub fn bar(h: impl Hei) {
//     h.hei()
// }

// pub fn foo() {
//     bar(&["J", "Jon"]);
//     bar(&[String::from("J"), String::from("Jon")]);
//     bar(&["J", String::from("Jon")]);
// }

// pub fn bar<H : Hei>(s: &[H]) {
//     for h in s {
//         h.hei();
//     }
// }

// pub fn bar(s: &[dyn Hei]) {
//     for h in s {
//         h.hei();
//     }
// }

// pub fn add_true(v: &mut dyn Extend<bool>) {
//     v.extend(std::iter::once(true));
// }

// x is not sized
// pub fn clone(v : &dyn Clone) {
//     let x = v.clone();
// }

pub fn it(v: &mut dyn Iterator<Item = bool>) {
    let _ = v.next();
}

pub fn drop(v: &mut dyn Drop) {
    // when v goes out of scope, Drop::drop is called
}

// vtable fro trait objects include Drop
