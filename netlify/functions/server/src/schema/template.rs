use async_graphql::{Enum, SimpleObject, ID};
use serde::Deserialize;
use url::Url;

#[derive(Clone, Debug, Deserialize, SimpleObject)]
/// Describes a single template that can be used to create a new subgraph via `rover template use`
pub(crate) struct Template {
    /// A unique identifier for the template to be used with the `--template` argument of `rover template use`
    pub(crate) id: ID,
    /// A short, human-readable name for the template.
    name: String,
    /// Indicates if the template is ready for initialization.
    description: String,
    /// Where the source code for this template can be found, along with a README describing how to use it.
    repo_url: Url,
    /// The URL where the template can be downloaded from
    download_url: Url,
    /// The programming language of the template
    pub(crate) language: Language,
}

#[derive(Clone, Copy, Debug, Deserialize, Enum, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum Language {
    CSharp,
    Go,
    Java,
    Javascript,
    Kotlin,
    Python,
    Rust,
    Typescript,
    Graphql
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_graphql_language_support() {
        let json_data = r#"
        [
            {
                "id": "template",
                "name": "Template",
                "description": "This is a template",
                "repo_url": "https://example.com/repo",
                "download_url": "https://example.com/download",
                "language": "GRAPHQL"
            }
        ]
        "#;

        let templates: Vec<Template> = serde_json::from_str(json_data)
            .expect("Failed to parse templates.json into Template structs");

        // Ensure the second template has init_ready is set to true
        let template = &templates[1];
        assert_eq!(template.id, ID::from("template"));
        assert_eq!(template.language, Language::Graphql);
    }
}

