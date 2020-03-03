pub(crate) enum ComparisonResult<T> {
    LeftOnly(T),
    RightOnly(T),
    Both(T),
}

pub fn compare_projects(left: Project, right: Project) -> ComparisonResult<Project> {
    
}