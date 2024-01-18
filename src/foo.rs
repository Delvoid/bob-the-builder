#[derive(Debug, PartialEq)]
pub struct Foo {
    bar: String,
}

impl Foo {
    // helps users discover the builder =)
    pub fn builder() -> FooBuilder {
        FooBuilder::default()
    }
}

#[derive(Default)]
pub struct FooBuilder {
    bar: String,
}

impl FooBuilder {
    pub fn new() -> Self {
        Self {
            bar: String::from("bar"),
        }
    }

    pub fn name(mut self, bar: String) -> Self {
        self.bar = bar;
        self
    }

    pub fn build(self) -> Foo {
        Foo { bar: self.bar }
    }
}

#[test]
fn builder_test() {
    let foo = Foo {
        bar: String::from("Y"),
    };
    let foo_from_builder: Foo = FooBuilder::new().name(String::from("Y")).build();
    println!("{:#?}", foo_from_builder);
    assert_eq!(foo, foo_from_builder);
}
