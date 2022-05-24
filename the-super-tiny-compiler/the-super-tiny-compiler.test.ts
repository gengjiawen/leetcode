import {
  codeGenerator,
  parser,
  tokenizer,
  transformer,
} from './the-super-tiny-compiler'

const input = '(add 2 (subtract 4 2))'
const output = 'add(2, subtract(4, 2));'

it('tiny compiler', () => {
  const tokens = tokenizer(input)
  expect(tokens).toMatchSnapshot()
  const ast = parser(tokens)
  expect(ast).toMatchSnapshot()
  let newAst = transformer(ast)
  expect(newAst).toMatchSnapshot()
  let o = codeGenerator(newAst)
  expect(o).toBe(output)
})
