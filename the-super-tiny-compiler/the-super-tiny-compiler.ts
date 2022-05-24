const WHITESPACE: RegExp = /\s/
const NUMBERS: RegExp = /[0-9]/
const LETTERS: RegExp = /[a-z]/i

interface Token {
  type: string
  value: string
}

export function tokenizer(input: string): Token[] {
  let current = 0

  let tokens = []

  while (current < input.length) {
    let char = input[current]

    if (char === '(') {
      tokens.push({
        type: 'paren',
        value: '(',
      })

      current++

      continue
    }

    if (char === ')') {
      tokens.push({
        type: 'paren',
        value: ')',
      })
      current++
      continue
    }

    if (WHITESPACE.test(char)) {
      current++
      continue
    }

    if (NUMBERS.test(char)) {
      let value = ''

      while (NUMBERS.test(char)) {
        value += char
        char = input[++current]
      }

      tokens.push({ type: 'number', value })

      continue
    }

    if (char === '"') {
      let value = ''

      char = input[++current]

      while (char !== '"') {
        value += char
        char = input[++current]
      }

      char = input[++current]

      tokens.push({ type: 'string', value })

      continue
    }

    if (LETTERS.test(char)) {
      let value = ''

      while (LETTERS.test(char)) {
        value += char
        char = input[++current]
      }

      tokens.push({ type: 'name', value })

      continue
    }

    throw new TypeError(`I dont know what this character is: ${char}`)
  }

  return tokens
}

interface Node {
  type: string
  value?: string
  name?: string
  callee?: Node
  expression?: Node
  body?: Node[]
  params?: Node[]
  arguments?: Node[]
  _context?: Node[]
}

export function parser(tokens: Token[]): Node {
  let current = 0

  function walk(): Node {
    let token = tokens[current]

    if (token.type === 'number') {
      current++

      return {
        type: 'NumberLiteral',
        value: token.value,
      }
    }

    if (token.type === 'string') {
      current++

      return {
        type: 'StringLiteral',
        value: token.value,
      }
    }

    if (token.type === 'paren' && token.value === '(') {
      token = tokens[++current]

      let node: Node = {
        type: 'CallExpression',
        name: token.value,
        params: [],
      }

      token = tokens[++current]

      while (
        token.type !== 'paren' ||
        (token.type === 'paren' && token.value !== ')')
      ) {
        node.params.push(walk())
        token = tokens[current]
      }

      current++

      return node
    }

    throw new TypeError(token.type)
  }

  let ast: Node = {
    type: 'Program',
    body: [],
  }

  while (current < tokens.length) {
    ast.body.push(walk())
  }

  return ast
}

interface Visitor {
  [key: string]: {
    enter: (n: Node, p: Node) => void
    exit?: (n: Node, p: Node) => void
  }
}

export function traverser(ast: Node, visitor: Visitor) {
  function traverseArray(array: Node[], parent: Node | null) {
    array.forEach((child) => {
      traverseNode(child, parent)
    })
  }

  function traverseNode(node: Node, parent: Node | null) {
    let methods = visitor[node.type]

    if (methods && methods.enter) {
      methods.enter(node, parent)
    }

    switch (node.type) {
      case 'Program':
        traverseArray(node.body, node)
        break

      case 'CallExpression':
        traverseArray(node.params, node)
        break

      case 'NumberLiteral':
      case 'StringLiteral':
        break

      default:
        throw new TypeError(node.type)
    }

    if (methods && methods.exit) {
      methods.exit(node, parent)
    }
  }

  traverseNode(ast, null)
}

export function transformer(ast: Node) {
  let newAst: Node = {
    type: 'Program',
    body: [],
  }

  ast._context = newAst.body

  traverser(ast, {
    NumberLiteral: {
      enter(node, parent) {
        parent._context.push({
          type: 'NumberLiteral',
          value: node.value,
        })
      },
    },

    StringLiteral: {
      enter(node, parent) {
        parent._context.push({
          type: 'StringLiteral',
          value: node.value,
        })
      },
    },

    CallExpression: {
      enter(node, parent) {
        let expression: Node = {
          type: 'CallExpression',
          callee: {
            type: 'Identifier',
            name: node.name,
          },
          arguments: [],
        }

        node._context = expression.arguments

        if (parent.type !== 'CallExpression') {
          expression = {
            type: 'ExpressionStatement',
            expression: expression,
          }
        }

        parent._context.push(expression)
      },
    },
  })

  return newAst
}

export function codeGenerator(node: Node): string {
  switch (node.type) {
    case 'Program':
      return node.body.map(codeGenerator).join('\n')

    case 'ExpressionStatement':
      return `${codeGenerator(node.expression)};`

    case 'CallExpression':
      return `${codeGenerator(node.callee)}(${node.arguments
        .map(codeGenerator)
        .join(', ')})`

    case 'Identifier':
      return node.name

    case 'NumberLiteral':
      return node.value

    case 'StringLiteral':
      return `"${node.value}"`

    default:
      throw new TypeError(node.type)
  }
}

export function compiler(input: string) {
  let tokens = tokenizer(input)
  let ast = parser(tokens)
  let newAst = transformer(ast)
  let output = codeGenerator(newAst)

  return output
}
