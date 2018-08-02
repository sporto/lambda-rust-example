#[macro_use] extern crate maplit;

extern crate aws_lambda as lambda;

fn main() {
    // start the runtime, and return a greeting every time we are invoked
    lambda::start(|()| {
        let response = hashmap!{
            "body" => "Hello",
        };

        Ok(response)
    })
}
