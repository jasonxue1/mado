pub struct Rule {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub aliases: Vec<String>,
    pub check: fn() -> Vec<usize>,
}
