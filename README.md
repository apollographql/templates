# Apollo Templates

[![Netlify Status](https://api.netlify.com/api/v1/badges/127b7d4b-f18e-4224-8338-7fe427598569/deploy-status)](https://app.netlify.com/sites/apollo-templates/deploys)

This repository serves as a guide and reference for building new templates that will be accessible via [Rover].

## Contribution Guidelines

Templates that are submitted by the community are expected to be maintained by the community. Because of the difficulty in enforcing this, we are currently only accepting templates contributed by the maintainers of the primary libraries that the template is built around. The template's repository should be located under the same account or organization as the primary library. For example, a template for Apollo Server should be located under the `apollographql` organization.

In order to keep the template up to date with the latest template requirements, please follow releases on this repository. We will create a release each time a requirement changes or an optional feature is added.

Once your template meets all of the requirements below, contribute the template by [opening an issue](https://github.com/apollographql/templates/issues/new?assignees=dbanty&labels=&template=new_template.yml).

<!-- TODO: Link to registry -->

## Template Requirements

In order to be included in the list of templates that can be used with [Rover], a template must meet the following requirements:

1. Be located in an open source Git repository.
2. Include a `LICENSE` file in the root of the repository with one of the approved license formats:
   1. [MIT]
   2. [Apache 2.0]
3. Include a `README.md` file in the root of the repo which describes what the template does, and the next steps to take after running `rover template use` with the template. For an example, see the [Apollo Server template].
4. Include a `.github/workflows` directory with GitHub Actions workflows (see "Author Resources" below for examples) which:
   1. Execute tests on pull request
   2. Run schema checks against Apollo Studio on pull request
   3. Publish schema to Apollo Studio on push to the default branch
5. Implement the federated subgraph schema in `schema.graphql` to provide examples / boilerplate for consumers. You can optionally add more to the schema to show off more features of the library, but the schema file contains the minimum that must be implemented.
6. Include example tests which exercise all resolvers (query, mutation, and entity) in the schema. These tests must make GraphQL queries to the server, not simply call the resolver functions directly. Example queries which should be executed in tests are in `test_queries.graphql`.
7. Contain CORS rules compatible with Apollo Studio. The least permissive set is:
   1. `Access-Control-Allow-Origin: https://studio.apollographql.com`
   2. `Access-Control-Allow-Methods: POST`
   3. `Access-Control-Allow-Headers: Content-Type`
8. Be runnable via some start command immediately after cloning (and installing any required dependencies). In its simplest form, template usage is simply cloning the latest version of the repo. All templates are expected to be fully-functional GraphQL servers without any further editing for quick evaluation.

## Template Recommendations

1. Set up your code to be deployable on [Railway] with [no additional configuration](https://docs.railway.app/deploy/builds). Most Apollo tutorials currently direct users to [Railway] for deployment as an easy, free option. You can optionally add a [Deploy to Railway] button to your `README.md` file as an even quicker getting started option.
2. Configure [Renovate] or [Dependabot] to keep dependencies up to date more easily. Templates which fall too far out of date will be removed from the registry.

## Author Resources

Feel free to look at any of the existing templates for help building your own. Get the latest list via `rover template list`. Additionally, this repo contains a few sets of GitHub Actions workflows to get you started in the `workflows` directory.

[Rover]: https://github.com/apollographql/rover
[Apollo Server template]: https://github.com/apollographql/subgraph-template-javascript-apollo-server-boilerplate
[MIT]: https://opensource.org/licenses/MIT
[Apache 2.0]: https://opensource.org/licenses/Apache-2.0
[Railway]: https://railway.app
[Deploy to Railway]: https://railway.app/button
[Renovate]: https://docs.renovatebot.com
[Dependabot]: https://docs.github.com/en/code-security/supply-chain-security/keeping-your-dependencies-updated-automatically
