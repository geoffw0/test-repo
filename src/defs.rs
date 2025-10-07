
macro_rules! myMacro {
    () => {
        println!("Hello from myMacro.");
    };
}

pub(crate) use myMacro;
