use git2::Commit;

pub(crate) struct AnalyzeResult<'repo> {
    pub(crate) breaking: Vec<Commit<'repo>>,
    pub(crate) features: Vec<Commit<'repo>>,
    pub(crate) fixes: Vec<Commit<'repo>>,
    pub(crate) others: Vec<Commit<'repo>>,
}

impl AnalyzeResult<'_> {
    pub(crate) fn is_breaking(&self) -> bool {
        !self.breaking.is_empty()
    }

    pub(crate) fn is_minor(&self) -> bool {
        !self.features.is_empty()
    }

    pub(crate) fn is_patch(&self) -> bool {
        !self.fixes.is_empty()
    }
}

pub(crate) fn analyze(commits: Vec<Commit<'_>>) -> anyhow::Result<AnalyzeResult<'_>> {
    for commit in commits {
        println!("{:#?}", commit);
    }
    todo!()
}
