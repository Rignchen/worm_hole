mod init;
mod add;
mod remove;
mod list;
mod edit;
mod query;

pub type Init = init::Init;
pub type AddAlias = add::AddAlias;
pub type RemoveAlias = remove::RemoveAlias;
pub type ListAliases = list::ListAliases;
pub type Query = query::Query;
pub type EditAlias = edit::EditAlias;

