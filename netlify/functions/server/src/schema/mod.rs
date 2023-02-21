use lazy_static::lazy_static;
use crate::schema::template::{Language, Template};
use async_graphql::{EmptyMutation, EmptySubscription, Object, ID};

mod template;

pub(crate) struct Query;

const TEMPLATES_JSON: &'static str = include_str!("../../templates.json");

lazy_static! {
    static ref TEMPLATES: Vec<Template> = serde_json::from_str(TEMPLATES_JSON).unwrap();
    pub(crate) static ref SCHEMA: Schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .limit_complexity(100)
        .finish();
}

#[Object]
impl Query {
    /// Get an optionally filtered list of templates
    async fn templates(&self, language: Option<Language>) -> Vec<Template> {
        TEMPLATES
            .iter()
            .filter(|template| {
                if let Some(language) = language {
                    template.language == language
                } else {
                    true
                }
            })
            .cloned()
            .collect()
    }

    /// Get a template by ID
    async fn template(&self, id: ID) -> Option<Template> {
        get_template_by_id(id)
    }

    /// `@key(fields: "id")`
    #[graphql(entity)]
    async fn template_entity(&self, id: ID) -> Option<Template> {
        get_template_by_id(id)
    }
}

fn get_template_by_id(id: ID) -> Option<Template> {
    TEMPLATES
        .iter()
        .find(|template| template.id == id)
        .cloned()
}

type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;