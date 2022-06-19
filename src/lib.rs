#[derive(Default)]
struct A<Command> {
    a: Command,
}

impl<Command: Default> A<Command> {
    thread_local! {static AA: A<Command> = A::default() }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
