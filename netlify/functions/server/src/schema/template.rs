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
    /// An extended description of what the template does.
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
    Go,
    Java,
    Javascript,
    Kotlin,
    Python,
    Rust,
    Typescript,
}
