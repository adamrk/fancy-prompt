mod git;

pub enum VcsType {
    Git,
}

#[derive(Debug,Copy,Clone)]
pub enum ActiveOperation {
    None,
    Merge,
    Revert,
    CherryPick,
    Bisect,
    Rebase,
}

pub trait VcsInfo {
    fn vcs(&self) -> VcsType;
    fn has_modified_files(&self) -> bool;
    fn has_staged_files(&self) -> bool;
    fn has_new_files(&self) -> bool;
    fn has_commits(&self) -> bool;
    fn active_operation(&self) -> ActiveOperation;
    fn branch(&self) -> Option<String>;
    fn remote_branch_diff(&self) -> Option<(usize, usize)>;

    fn is_dirty(&self) -> bool {
        self.has_modified_files()
            || self.has_staged_files()
            || self.has_new_files()
            || !self.remote_branch_diff().is_some()
    }

    fn is_error(&self) -> bool {
        !self.has_commits()
    }
}

pub fn detect() -> Option<Box<VcsInfo>> {
    if let Some(git) = git::detect() {
        Some(git)
    }
    else {
        None
    }
}
