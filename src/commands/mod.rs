mod add;
mod edit;
mod init;
mod list;
mod query;
mod remove;
mod rename;

pub type Init = init::Init;
pub type AddAlias = add::AddAlias;
pub type RemoveAlias = remove::RemoveAlias;
pub type ListAliases = list::ListAliases;
pub type Query = query::Query;
pub type EditAlias = edit::EditAlias;
pub type RenameAlias = rename::RenameAlias;
