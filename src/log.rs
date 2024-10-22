use console::style;

pub(crate) enum Category {
    Cmd,
    Arg,
    Info,
    Warn,
    Ok,
    Err,
}

fn style_input(input: &str, category: Category) -> String {
    match category {
        Category::Cmd => style(input).cyan(),
        Category::Arg => style(input).magenta(),
        Category::Info => style(input).blue(),
        Category::Ok => style(input).green(),
        Category::Warn => style(input).yellow(),
        Category::Err => style(input).red(),
    }
    .to_string()
}

pub(crate) fn log(input: &str) {
    print!("{}", input);
}

pub(crate) fn logc(input: &str, category: Category) {
    print!("{}", style_input(input, category));
}

pub(crate) fn logln(input: &str) {
    println!("{}", input);
}

pub(crate) fn logcln(input: &str, category: Category) {
    println!("{}", style_input(input, category));
}
