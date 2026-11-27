#[derive(Debug, Default, Clone, clap::Args)]
pub struct TagFilter {
    /// Require this exact tag (repeat to require all).
    #[arg(long = "tag", value_name = "TAG")]
    required: Vec<String>,
    /// Exclude results with this exact tag (repeat to exclude any).
    #[arg(long = "exclude-tag", value_name = "TAG")]
    excluded: Vec<String>,
}

impl TagFilter {
    pub fn matches(&self, has_tag: impl Fn(&str) -> bool) -> bool {
        self.required.iter().all(|tag| has_tag(tag))
            && !self.excluded.iter().any(|tag| has_tag(tag))
    }
}
