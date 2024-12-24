use String;

#[derive(Debug)]
struct Todo {
    task: String,
    checked: bool
}

// This does not work.
// fn todo_push(todos: &mut Vec<&Todo>, todo: &Todo) {
// This works, because compiler now knows they both have the same lifetime.
// fn todo_push<T>(todos: &mut Vec<T>, todo: T) {
// This works as well, because now we are explicitly saying that they have
// the same lifetime.
fn todo_push<'a>(todos: &mut Vec<&'a Todo>, todo: &'a Todo) {
// This also does the same thing, but lifetime 'c is not related to any other
// arguments, so no need to explicitly state it.
// fn todo_push<'a, 'c>(todos: &'c mut Vec<&'a Todo>, todo: &'a Todo) {
    // This statement makes it a requirment for lifetime of Todo struct in a Vec 
    // and todo struct itself to be the same.
    todos.push(todo);
}
// This how we can specify an object's lifetime.
// References:
// <https://www.youtube.com/watch?v=-gkvOoxgp8E>
// <https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision>
// <https://doc.rust-lang.org/reference/lifetime-elision.html>
// <https://doc.rust-lang.org/reference/trait-bounds.html#lifetime-bounds>

fn main() {
    let mut str1 = String::from("Hello");
    str1.push_str(" from Rust!!");
    println!("{}", str1);
    // It errors out for the bottom line. Because it moves the value of str2 to str1.
    // So when the coder tries to access str1, it causes an error. For non-primitive
    // types, rust uses move semantics instead of copy. If the coder wants to create
    // a copy, coder should make a explicit copy/clone.
    // let str2 = str1;
    // I can borrow the value, as the borrowed ref does not outlive the original value.
    // But this still causes an error, println borrows an immutable reference to str1.
    // A data can only have one mutable reference, or multiple immutable references.
    // let str2 = &mut str1;
    let str2 = &str1;
    println!("{} {}", str1, str2);

    let mut todo1 = Todo{task: String::from("task1"), checked: false};
    let todo2 = &mut todo1;
    todo2.checked = true;
    // Technically println creates an immutable borrow for todo1, but this does not
    // cause any error as long as we are to referencing todo2, which is a mutable
    // reference to todo1.
    println!("{:?}", todo1);
    let todo3 = Todo{task: String::from("task2"), checked: false};
    let todo4 = Todo{task: String::from("task3"), checked: false};
    let mut todos: Vec<&Todo> = vec![];
    todo_push(&mut todos, &todo3);
    todo_push(&mut todos, &todo4);
    println!("{:?}", todos);
}
