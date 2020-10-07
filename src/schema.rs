use juniper::FieldResult;
use juniper::RootNode;

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn api_version() -> &str {
        "0.1.0"
    }

    fn debug(api_key: String) -> String {
        api_key
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {

}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
