use attr_bounds::bounds;

#[bounds(T Clone)]
struct Foo<T>(T);

fn main() {}
