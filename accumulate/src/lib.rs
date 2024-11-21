pub fn map<T, U>(input: Vec<T>, mut function: impl FnMut(T) -> U) -> Vec<U> {
    let mut output = Vec::with_capacity(input.len());

    for item in input {
        output.push(function(item));
    }

    output
}
