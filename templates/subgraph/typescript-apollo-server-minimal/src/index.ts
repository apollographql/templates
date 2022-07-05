
import { resolve } from 'path';
import { readFileSync } from 'fs';

import gql from 'graphql-tag';
import { ApolloServer } from 'apollo-server';
import { buildSubgraphSchema } from '@apollo/subgraph';

import resolvers from './resolvers';

const PORT = 4001;
const shouldMock = process.env.SHOULD_MOCK === 'true';

async function main() {
  let typeDefs = gql(
    readFileSync(resolve('..', 'schema.graphql'), {
      encoding: 'utf-8',
    }),
  );

  const server = new ApolloServer({
    schema: buildSubgraphSchema({ typeDefs, resolvers }),
    mocks: shouldMock,
    mockEntireSchema: false,
    context: ({ req }) => {
      return {
        authorization: req?.headers['authorization'] ?? '',
      };
    },
  });

  await server
    .listen({ port: PORT })
    .then(({ url }) => console.log(`Subgraph ready at: ${url}`));
}

main();
