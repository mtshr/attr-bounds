use attr_bounds::bounds;

#[bounds(T: Clone)]
mod foo {
    struct Bar;
}

fn main() {}
