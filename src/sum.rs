pub fn sum_fn_parsed<'a, I, J, F>(vals: I, fun: F) -> i32
where
    I: IntoIterator<Item = &'a J>,
    J: AsRef<str> + 'a,
    F: Fn(i32) -> i32,
{
    vals.into_iter()
        .map(AsRef::as_ref)
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .map(fun)
        .sum()
}
