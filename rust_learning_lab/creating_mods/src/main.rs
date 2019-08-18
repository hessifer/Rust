mod messages;  // name of file containing module
use messages::greetings::say_hello; // module_file -> module_name -> function

fn main() {
    say_hello();
}
