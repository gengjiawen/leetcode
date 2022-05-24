import axios from 'axios'
import Sequelize from 'sequelize'
import * as path from 'path'
import async from 'async'

const sequelize = new Sequelize.Sequelize('leetcode', 'null', 'null', {
  dialect: 'sqlite',
  define: {
    timestamps: true,
    freezeTableName: true,
  },
  storage: path.normalize(`${__dirname}/leetcode.db`),
})

export const Problem = sequelize.define('leetcode', {
  id: { type: Sequelize.INTEGER, primaryKey: true },
  frontend_id: Sequelize.INTEGER,
  url: Sequelize.STRING,
  content: Sequelize.STRING,
  codeDefinition: { type: Sequelize.STRING },
  title: { type: Sequelize.STRING },
  difficulty: { type: Sequelize.INTEGER },
  paidOnly: { type: Sequelize.BOOLEAN },
})

Problem.sync()

async function getQuestionDetail(title_slug: string) {
  const url = `https://leetcode.cn/graphql`
  return await axios.post(
    url,
    {
      operationName: 'getQuestionDetail',
      variables: { titleSlug: title_slug },
      query: `
          query getQuestionDetail($titleSlug: String!) {
  isCurrentUserAuthenticated
  question(titleSlug: $titleSlug) {
    questionId
    questionFrontendId
    questionTitle
    questionTitleSlug
    content
    codeDefinition
  }
}
`,
    },
    {
      headers: {
        'Content-Type': 'application/json',
        Referer: 'https://leetcode.com/',
        'User-Agent':
          'Mozilla/5.0 (Macintosh; Intel Mac OS X 10.13; rv:52.0) Gecko/20100101 Firefox/52.0',
        'Cache-Control': 'no-cache',
      },
    },
  )
}

export async function saveRecord() {
  const r = await axios.get('https://leetcode.com/api/problems/algorithms/')
  const response = r.data
  return new Promise((resolve, reject) => {
    async.mapLimit(
      response.stat_status_pairs,
      5,
      async function (item: any) {
        const url = `https://leetcode.com/problems/${item.stat.question__title_slug}`
        const r = await Problem.findOne({ where: { url: url } })
        if (r !== null) {
          return r
        }
        const raw = await getQuestionDetail(item.stat.question__title_slug)
        const result = await Problem.upsert({
          id: item.stat.question_id,
          url: url,
          frontend_id: item.stat.frontend_question_id,
          content: raw.data.data.question.content,
          codeDefinition: raw.data.data.question.codeDefinition,
          title: item.stat.question__title,
          difficulty: item.difficulty.level,
          paidOnly: item.paid_only,
        })
        return result
      },
      (err, results) => {
        if (err) reject(err)
        // results is now an array of the response bodies
        resolve(results)
      },
    )
  })
}
