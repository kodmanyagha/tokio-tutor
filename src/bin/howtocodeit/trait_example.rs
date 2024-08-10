use std::path::Path;

use anyhow::anyhow;

pub trait FooTrait {
    fn foo();
    fn bar(&self);
}

pub struct ExampleStruct1<'a> {
    pub prop1: i32,
    pub prop2: &'a str,
}

pub fn example_fnonce_1<F>(p: F)
where
    F: FnOnce(i32, i32) -> i32,
{
    let result = p(10, 20);

    println!(">> result: {result}");
}

impl<'a> FooTrait for ExampleStruct1<'a> {
    fn foo() {
        let x = |p1: i32| println!(">> p1: {}", p1);
        x(10);

        let x = move |p1: i32| p1 * p1;
        println!(">> x result: {}", x(10));

        example_fnonce_1(|p1, p2| p1 + p2);

        let x: Option<i32> = Some(1);
        let x: Result<i32, &str> = x.ok_or_else(|| "no number provided");
        println!("foo: {:?}", x);

        let nums = vec![1, 2, 3];
        let doubled: Vec<_> = nums.iter().map(|n| n * 2).collect();
        let doubled = nums.iter().map(|n| n * 2).collect::<Vec<_>>();

        println!("foo function invoked.");
    }

    fn bar(&self) {
        println!("prop1: {}", self.prop1);
        println!("prop2: {}", self.prop2);
    }
}

impl<'a> ExampleStruct1<'a> {
    pub fn asref_example<P: AsRef<Path>>(&self, p: P) -> anyhow::Result<()> {
        let path_str = p
            .as_ref()
            .to_str()
            .ok_or_else(|| anyhow!("Can't convert to string."))?;

        println!("Path str: {}", path_str);

        Ok(())
    }
}

pub fn run_trait_example() -> anyhow::Result<()> {
    let s1 = ExampleStruct1 {
        prop1: 10,
        prop2: "test",
    };

    s1.bar();
    ExampleStruct1::foo();
    let _ = s1.asref_example("/path/to/here");
    let _ = s1.asref_example(Path::new("/foo/bar/baz"));
    let _ = s1.asref_example("/example/str/path".to_string());

    Ok(())
}
