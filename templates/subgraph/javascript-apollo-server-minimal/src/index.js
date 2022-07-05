const { resolve } = require('path');
const { readFileSync } = require('fs');

const gql = require('graphql-tag');
const { ApolloServer } = require('apollo-server');
const { buildSubgraphSchema } = require('@apollo/subgraph');

const PORT = 4001;
const shouldMock = process.env.SHOULD_MOCK === 'true';

async function main() {
  const typeDefs = gql(
    readFileSync(resolve('..', 'schema.graphql'), {
      encoding: 'utf-8',
    }),
  );

  const server = new ApolloServer({
    schema: buildSubgraphSchema({ typeDefs }),
    mocks: shouldMock,
    mockEntireSchema: false,
  });

  await server
    .listen({ port: PORT })
    .then(({ url }) => console.log(`Subgraph ready at: ${url}`));
}

main();
