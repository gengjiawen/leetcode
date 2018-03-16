import * as toMarkdown from "to-markdown";
import * as fs from "fs-extra";
import {Problem} from "./crawl_leetcode";
import {Converter} from "to-markdown";

const converter_code: Converter = {
  filter: ['pre'],
  replacement: function(content, node) {
    return '\n```\n' + content + '```\n'
  },
}


function underline(s: string) {
  return s
    .split(' ')
    .map(a => a.toLowerCase())
    .join('_')
    .replace("?", "")
}

interface CodeConfig {
  getCode(code: any[]): string;
  ext: string;
  comment: string;
  fileNameStyle(s: string): string;
}
const JSconfig: CodeConfig = {
  getCode: c => c.find(a => a.value === 'javascript').defaultCode,
  ext: 'js',
  comment: '//',
  fileNameStyle: s => s.replace(/[\s|?]/g, '')
}

const Goconfig: CodeConfig = {
  getCode: c => c.find(a => a.value === 'golang').defaultCode,
  ext: 'go',
  comment: '//',
  fileNameStyle: underline
}

function generatePool(config: CodeConfig) {
  fs.ensureDirSync('pools')
  Problem.findAll().then((records: any[]) => {
    records.forEach(i => {
      const p = i.get()
      //bypass paidOnly
      if (p.paidOnly || p.urlContent === null) {
        return
      }

      const data = JSON.parse(p.urlContent).data
      const { getCode, ext, comment, fileNameStyle } = config
      let code = undefined
      try {
        code = getCode(JSON.parse(data.question.codeDefinition))
      } catch (e) {
        return
      }
      const name = `pools/${fileNameStyle(p.title)}.${ext}`
      const content = data.question.content
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

      console.log('generate pools finished')
    })
  })
}


generatePool(JSconfig)
