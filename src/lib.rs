use std::fmt::Write;

#[derive(PartialEq, Eq)]
pub struct Roff {
    title: String,
    section: i8,
    content: Vec<Section>,
}

impl Roff {
    pub fn new(title: &str, section: i8) -> Self {
        Roff {
            title: title.into(),
            section,
            content: Vec::new(),
        }
    }

    pub fn section<'a, C, I>(mut self, title: &str, content: I) -> Self
    where
        I: IntoIterator<Item = &'a C>,
        C: Troffable + 'a,
    {
        let title = title.into();
        let content = content.into_iter().map(|x| x.render()).collect();

        self.content.push(Section { title, content });
        self
    }
}

impl Troffable for Roff {
    fn render(&self) -> String {
        let mut res = String::new();

        writeln!(
            &mut res,
            ".TH {} {}",
            self.title.to_uppercase(),
            self.section
        )
        .unwrap();
        for section in &self.content {
            writeln!(&mut res, "{}", escape(&section.render())).unwrap();
        }

        res
    }
}

#[derive(PartialEq, Eq)]
struct Section {
    title: String,
    content: String,
}

impl Troffable for Section {
    fn render(&self) -> String {
        let mut res = String::new();

        writeln!(&mut res, ".SH {}", self.title.to_uppercase()).unwrap();
        res.push_str(&self.content);

        res
    }
}

pub trait Troffable {
    fn render(&self) -> String;
}

impl Troffable for String {
    fn render(&self) -> String {
        self.clone()
    }
}

impl<'a> Troffable for &'a str {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl<'a, C: Troffable> Troffable for &'a [C] {
    fn render(&self) -> String {
        self.iter().map(Troffable::render).collect()
    }
}

impl<C: Troffable> Troffable for Vec<C> {
    fn render(&self) -> String {
        self.iter().map(Troffable::render).collect()
    }
}

pub fn bold(input: &str) -> String {
    format!(r"\fB{}\fP", input)
}

pub fn italic(input: &str) -> String {
    format!(r"\fI{}\fP", input)
}

pub fn list<C1: Troffable, C2: Troffable>(header: &'_ [C1], content: &'_ [C2]) -> String {
    format!(".TP\n{}\n{}", header.render(), content.render())
}

fn escape(input: &str) -> String {
    input.replace("-", r"\-")
}
