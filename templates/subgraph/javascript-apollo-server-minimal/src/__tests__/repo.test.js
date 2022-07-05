const { buildSubgraphSchema } = require('@apollo/subgraph');
const { ApolloServer } = require('apollo-server');
const { readFileSync } = require('fs');
const gql = require('graphql-tag');
const { resolve } = require('path');

let mockedSubgraph;

describe('Repository Template Functionality', () => {
  beforeAll(async () => {
    let typeDefs = gql(
      readFileSync(resolve('schema.graphql'), {
        encoding: 'utf-8',
      }),
    );

    const server = new ApolloServer({
      schema: buildSubgraphSchema({ typeDefs }),
      mocks: true,
      mockEntireSchema: false,
    });
    await server.listen();
    mockedSubgraph = server;
  });
  afterAll(async () => {
    await mockedSubgraph.stop();
  });

  it('Execute root query', async () => {
    //Arrange
    const query = 'query { foo(id:"1") { name } }';
    const expected = { foo: { name: 'Hello World' } };

    //Act
    const res = await mockedSubgraph.executeOperation({ query });

    //Assert
    expect(res.data).toEqual(expected);
  });
});
