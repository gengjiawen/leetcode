import * as toMarkdown from 'to-markdown'
import {Converter} from 'to-markdown'
import * as fs from 'fs-extra'
import {Problem} from './crawl_leetcode'

const converter_code: Converter = {
  filter: ['pre'],
  replacement: function(content, node) {
    return '\n```\n' + content + '```\n'
  },
}

export function underline(s: string) {
  return s
      .split(' ')
      .map(a => a.toLowerCase())
      .join('_')
      .replaceAll('?', '')
      .replaceAll('-', '_')
}

interface CodeConfig {
  getCode(code: any[]): string
  ext: string
  comment: string

  fileNameStyle(s: string, id: number): string
}

export const JSconfig: CodeConfig = {
  getCode: c => c.find(a => a.value === 'javascript').defaultCode,
  ext: 'js',
  comment: '//',
  fileNameStyle: s => s.replace(/[\s|?]/g, ''),
}

export const Goconfig: CodeConfig = {
  getCode: c => c.find(a => a.value === 'golang').defaultCode,
  ext: 'go',
  comment: '//',
  fileNameStyle: underline,
}

export const Rust: CodeConfig = {
  getCode: c => c.find(a => a.value === 'rust').defaultCode,
  ext: 'rs',
  comment: '//',
  fileNameStyle: (s, i) => `_${i.toString().padStart(4, '0')}_${underline(s)}`,
}

function generatePool(config: CodeConfig) {
  fs.emptydirSync('pools')
  Problem.findAll().then((records) => {
    records.forEach(i => {
      const p = i.get()
      //bypass paidOnly
      if (p.paidOnly) {
        return
      }

      const {getCode, ext, comment, fileNameStyle} = config
      let code: string;
      try {
        code = getCode(JSON.parse(p.codeDefinition));
      } catch (e) {
        // not all languages have codeDefinition
        return
      }
      const name = `pools/${fileNameStyle(p.title, p.frontend_id)}.${ext}`
      const content = p.content
      let description = toMarkdown(content, {
        gfm: true,
        converters: [
          {
            filter: ['div'],
            replacement: function(content) {
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

      const fileContent = `${comment} ${p.url}\n${description}\n\n${code}`

      fs.writeFile(name, fileContent, err => {
        if (err) throw err
      })
    })
  })
}

console.log('generate pools finished')
generatePool(Rust)
