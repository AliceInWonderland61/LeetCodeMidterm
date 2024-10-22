fn create_hello_world()->impl Fn() ->String{
    || String::from ("Hello World")
}

fn main()
{
    let function=create_hello_world();
    println!("{}",function());
}