import * as toMarkdown from 'to-markdown'
import { Converter } from 'to-markdown'
import * as fs from 'fs-extra'
import {Problem} from './crawl_leetcode'
import {extract_rust_solution, underline} from './utils'

const converter_code: Converter = {
  filter: ['pre'],
  replacement: function (content, node) {
    return '\n```\n' + content + '```\n'
  },
}

interface CodeConfig {
  getCode(code: any[]): string

  ext: string
  comment: string

  fileNameStyle(s: string, id: number): string

  patchCode?: (code: string) => string
  test?: (meta: TestMeta) => void
}

export const JSconfig: CodeConfig = {
  getCode: (c) => c.find((a) => a.value === 'javascript').defaultCode,
  ext: 'js',
  comment: '//',
  fileNameStyle: (s) => s.replace(/[\s|?]/g, ''),
}

export const Goconfig: CodeConfig = {
  getCode: (c) => c.find((a) => a.value === 'golang').defaultCode,
  ext: 'go',
  comment: '//',
  fileNameStyle: underline,
}

export interface TestMeta {
  code: string
  fileName?: string
}

export const Rust: CodeConfig = {
  getCode: (c) => c.find((a) => a.value === 'rust').defaultCode,
  ext: 'rs',
  comment: '//',
  fileNameStyle: (s, i) => `_${i.toString().padStart(4, '0')}_${underline(s)}`,
  patchCode: extract_rust_solution,
  test: (meta) => {
    const {code} = meta
    meta.code = `${code}
#[test]
pub fn t1() {
}
`
    return meta
  }
}

async function generatePool(config: CodeConfig, test: boolean = false) {
  fs.emptydirSync('pools')
  const problems = await Problem.findAll({
    limit: test ? 1 : undefined,
  })
  problems.forEach((i) => {
    const p = i.get()
    const {getCode, ext, comment, fileNameStyle} = config
    const file_name = `pools/${fileNameStyle(p.title, p.frontend_id)}.${ext}`
    if (p.paidOnly) {
      const fileContent = `${comment} ${p.url}\n`

      fs.writeFile(file_name, fileContent, (err) => {
        if (err) throw err
      })
      return
    }

    let code: string
    try {
      code = getCode(JSON.parse(p.codeDefinition))
    } catch (e) {
      // not all languages have codeDefinition
      return
    }
    const content = p.content
    let description = toMarkdown(content, {
      gfm: true,
      converters: [
        {
          filter: ['div'],
          replacement: function (content) {
            return content
          },
        },
        converter_code,
      ],
    })
    description = description
        .trim()
        .split('\n')
        .reduce((sum, value) => {
          return `${sum}\n${comment} ${value}`
        }, `${comment} `)

    let codeContent = config.patchCode ? config.patchCode(code) : code
    const finalFile = {
      codeContent
    }
    if (config.test) {
      const t = {code: codeContent}
      config.test(t)
      codeContent = t.code
    }
    const fileContent = `${comment} ${p.url}\n${description}\n\n${codeContent}`

    fs.writeFile(file_name, fileContent, (err) => {
      if (err) throw err
    })
  })
}

console.log('generate pools finished')
let test = false
process.argv.includes('--test') && (test = true)
generatePool(Rust, test)
