use attr_bounds::bounds;

#[bounds(T: Copy)]
struct Foo<T>(T);

fn main() {
    let _foo = Foo(Vec::new());
}
