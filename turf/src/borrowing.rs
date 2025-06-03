#![allow(unused)]
mod datatypes;

fn main(){
    datatypes::datatypes();

    let mut foo: Foo = Foo { x: 5 };
    let ref_foo: &mut Foo = &mut foo; // cannot reference a mutable variable to a mutable reference while it is borrowed

    print_foo(ref_foo);
    print_foo(&mut foo);
}

struct Foo{
    x: i32,
}

fn print_foo(foo: &mut Foo) {
    foo.x += 1;
    println!("Foo x: {}", foo.x);
}