const Sequelize = require('sequelize')
const toMarkdown = require("to-markdown")
const axios = require('axios')
const fs = require('fs')
const cheerio = require('cheerio')

const sequelize = new Sequelize('sqlite://leetcode.db', {
  define: {
    timestamps: true,
    freezeTableName: true,
    logging: false
  }
})

const Problem = sequelize.define('leetcode', {
  id: {type: Sequelize.INTEGER, primaryKey: true},
  url: {type: Sequelize.STRING},
  urlContent: {type: Sequelize.STRING},
  title: {type: Sequelize.STRING},
  difficulty: {type: Sequelize.INTEGER},
  paidOnly: {type: Sequelize.BOOLEAN}
})

Problem.sync()

async function saveRecord () {
  const r = await axios.get('https://leetcode.com/api/problems/algorithms/')
  const response = r.data
  response.stat_status_pairs.forEach(async item => {
    const url = `https://leetcode.com/problems/${item.stat.question__title_slug}`
    console.log(url)
    const raw = await axios.get(url, {
      timeout: 5000
    })
    const urlContent = raw.data
    Problem.upsert({
      id: item.stat.question_id,
      url: url,
      urlContent: urlContent,
      title: item.stat.question__title,
      difficulty: item.difficulty.level,
      paidOnly: item.paid_only
    })
  })

  return response
}

// uncomment this below to download the db
// saveRecord()
//   .then(r => {
//     console.log(r)
//   })
//   .catch(e => {
//     console.log(e)
//   })

const converter_code = {
  filter: ['pre'],
  replacement: function (content, node) {
    return "\n```\n" + content + "```\n";
  }
}

function generatePool () {
  Problem.findAll()
    .then(records => {
      records.forEach(i => {
        const p = i.get()
        //bypass paidOnly
        if (p.paidOnly || p.urlContent === null) {
          return
        }
        const $ = cheerio.load(p.urlContent)
        $('script').each((i, a) => {
          const s = a.children[0]
          if (s !== undefined && s.data !== undefined) {
            if (s.data.trim().startsWith("var pageData")) {
              eval(s.data.trim())
              let python = pageData.codeDefinition.find(a => a.value === 'python3')
              if (python === undefined) {
                python = pageData.codeDefinition.find(a => a.value === 'python')
              }

              const code = python.defaultCode;
              const name = `pools/${p.title.replace(/\s/g, '')}.py`;
              const content = $(".question-description");
              let description = toMarkdown(content.html(), { gfm: true,
                converters: [{
                  filter: ["div",],
                  replacement: function (content) { return content },
                }, converter_code],
              });
              description = description.trim()
                .split("\n")
                .filter(a => a.trim() !== "")
                .reduce((sum, value) => {
                return `${sum}\n# ${value.trim()}`
              }, "# ")
              console.log(description);

              const fileContent = `# ${p.url}\n${description}\n\n${code}`
              fs.writeFile(name, fileContent, (err) => {
                if (err) throw err
                console.log('The file has been saved!')
              })
              
            }
          }
        })
      })
    })
}

generatePool();
