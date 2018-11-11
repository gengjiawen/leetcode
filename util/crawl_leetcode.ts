import axios from 'axios'
import Sequelize from 'sequelize'
import * as path from 'path'

const sequelize = new Sequelize('leetcode', 'null', 'null', {
  dialect: 'sqlite',
  define: {
    timestamps: true,
    freezeTableName: true,
  },
  storage: path.normalize(`${__dirname}/leetcode.db`),
})

export const Problem = sequelize.define('leetcode', {
  id: { type: Sequelize.INTEGER, primaryKey: true },
  url: { type: Sequelize.STRING },
  urlContent: { type: Sequelize.STRING },
  title: { type: Sequelize.STRING },
  difficulty: { type: Sequelize.INTEGER },
  paidOnly: { type: Sequelize.BOOLEAN },
})

Problem.sync()

async function getQuestionDetail(title_slug: string) {
  const url = `https://leetcode.com/graphql`
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
    difficulty
    stats
    contributors
    similarQuestions
    discussUrl
    mysqlSchemas
    randomQuestionUrl
    sessionId
    categoryTitle
    submitUrl
    interpretUrl
    codeDefinition
    sampleTestCase
    enableTestMode
    metaData
    enableRunCode
    enableSubmit
    judgerAvailable
    infoVerified
    envInfo
    urlManager
    article
    questionDetailUrl
    discussCategoryId
    discussSolutionCategoryId
    libraryUrl
    companyTags {
      name
      slug
      translatedName
    }
    topicTags {
      name
      slug
      translatedName
    }
  }
  subscribeUrl
  isPremium
  loginUrl
}
`,
    },
    {
      headers: {
        'Content-Type': 'application/json',
        'x-csrftoken': 'SVU7oF1p6tpUnCV2RAbTDgMDWmDhoMyVUrvZT5fQBIkCBlHKB4ZEqiXiuoS1VEwA',
        Cookie:
          '__cfduid=d9a16b6a9230fc9ac5336daa9d028b3aa1518086485; csrftoken=SVU7oF1p6tpUnCV2RAbTDgMDWmDhoMyVUrvZT5fQBIkCBlHKB4ZEqiXiuoS1VEwA; _ga=GA1.2.386088192.1518086487; LEETCODE_SESSION=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6InRlY2huaWNhbGN1dGUiLCJ1c2VyX3NsdWciOiJ0ZWNobmljYWxjdXRlIiwiX2F1dGhfdXNlcl9pZCI6Ijg3MTM1IiwiUkVNT1RFX0FERFIiOiIxNC4yMC44OS4yMTIiLCJ0aW1lc3RhbXAiOiIyMDE4LTAyLTI4IDAyOjI5OjM3Ljg3MDQ1MiswMDowMCIsIl9hdXRoX3VzZXJfYmFja2VuZCI6ImFsbGF1dGguYWNjb3VudC5hdXRoX2JhY2tlbmRzLkF1dGhlbnRpY2F0aW9uQmFja2VuZCIsImlkIjo4NzEzNSwiYXZhdGFyIjoiaHR0cHM6Ly93d3cuZ3JhdmF0YXIuY29tL2F2YXRhci9lZGJjMTUyODgxOWRlZTdmZmU5MzAxZjQwNmMxYjgzYy5wbmc_cz0yMDAiLCJfYXV0aF91c2VyX2hhc2giOiI1NmIwZWM5NWE4MTdkMTQ5MDFlNDQwNTlmMGUxMTg2ZDk5YWQ5ZjI3IiwiZW1haWwiOiJ0ZWNobmljYWxjdXRlQGdtYWlsLmNvbSIsIklERU5USVRZIjoiZTZiYjVjZjk1OWQyMTQ2OWVhMjEzY2QzNDRkNjI5ZWEifQ.qxsM6lRE0qxOtiN4sV2HebS0MmFX44QCcS3ZVKVAlI4; _gid=GA1.2.2127778503.1519968442; _gat=1',
        Referer: 'https://leetcode.com/',
        'User-Agent':
          'Mozilla/5.0 (Macintosh; Intel Mac OS X 10.13; rv:52.0) Gecko/20100101 Firefox/52.0',
        'Cache-Control': 'no-cache',
      },
    }
  )
}

async function saveRecord() {
  const r = await axios.get('https://leetcode.com/api/problems/algorithms/')
  const response = r.data
  response.stat_status_pairs.forEach(async (item: any) => {
    console.log(item)
    const url = `https://leetcode.com/problems/${item.stat.question__title_slug}`
    const r = await Problem.findOne({ where: { url: url } })
    if (r !== null) {
      return
    }
    const raw = await getQuestionDetail(item.stat.question__title_slug)
    const urlContent = JSON.stringify(raw.data)
    console.log(urlContent)
    Problem.upsert({
      id: item.stat.question_id,
      url: url,
      urlContent: urlContent,
      title: item.stat.question__title,
      difficulty: item.difficulty.level,
      paidOnly: item.paid_only,
    })
  })

  return response
}
