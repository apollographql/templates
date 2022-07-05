import { Resolvers } from '../__generated__/resolvers-types';

export const resolvers: Resolvers = {
  Query: {
    foo(_parent, { id }, _context) {
      return {id, name: "FooBar"};
    },
  },
  Foo: {
    __resolveReference(foo, _context) {
      return {id: foo.id, name: "FooBar"};
    },
  },
};
