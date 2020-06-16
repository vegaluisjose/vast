use pretty::RcDoc;

pub const PRETTY_DEFAULT_WIDTH: usize = 100;

pub trait PrettyPrinter {
    fn to_doc(&self) -> RcDoc<()>;

    fn to_pretty_with_width(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }

    fn to_pretty(&self) -> String {
        self.to_pretty_with_width(PRETTY_DEFAULT_WIDTH)
    }
}
