use clap::{Parser, Subcommand, Args};

/// Manage your project and issues
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create, edit, list, delete, close issue
    #[command(subcommand)]
    Issue(IssueCommand),

    // /// Create, edit, list, delete, close epic
    // Epic(EpicCommand),
    // /// Create, edit, list, delete, close initiative
    // Initiative(InitiativeCommand),
}

#[derive(Debug, Subcommand)]
pub enum IssueCommand {
    /// Creates a new issue
    Create(CreateIssue),
    /// Lists all issues
    List(ListIssues),
    /// Update an issue to the repository (adds and commits with git)
    Up(UpIssue),
    /// Closes, adds and commits an issue
    Close(CloseIssue),
    // Edit(EditIssue),
    // Delete(DeleteIssue),
}

#[derive(Debug, Args)]
pub struct ListIssues {}

#[derive(Debug, Args)]
pub struct CreateIssue {
    /// Name of the issues
    #[arg(value_parser = is_not_empty)]
    pub name: String,
    ///
    /// If flag is set, the issue will be updated/registered to the repository (git add and commit)
    #[arg(long, short)]
    pub update: bool,
}

#[derive(Debug, Args)]
pub struct UpIssue {
    /// Name of the issues
    #[arg(value_parser = is_not_empty)]
    pub path: String,
}

#[derive(Debug, Args)]
pub struct CloseIssue {
    /// Name of the issues
    #[arg(value_parser = is_not_empty)]
    pub path: String,

}

fn is_not_empty(arg: &str) -> Result<String, String> {
    if arg.is_empty() {
        return Err("issue create: name cannot be empty".to_string());
    }
    Ok(arg.to_string())
}
