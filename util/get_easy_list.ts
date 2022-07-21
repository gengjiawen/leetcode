import axios from 'axios'
import * as dotenv from 'dotenv'
import * as fs from 'fs-extra'
import * as path from 'path'
import { Problem } from './crawl_leetcode'
import { Op } from 'sequelize'
import { generatePool } from './pools_utils'

dotenv.config()

export function getEasyList() {
  let data = JSON.stringify({
    query: `
    query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) {
  problemsetQuestionList(
    categorySlug: $categorySlug
    limit: $limit
    skip: $skip
    filters: $filters
  ) {
    hasMore
    total
    questions {
      frontendQuestionId
      solutionNum
      acRate
      difficulty
      freqBar
      isFavor
      paidOnly
      status
      title
    }
  }
}
    `,
    variables: {
      categorySlug: 'algorithms',
      skip: 0,
      limit: 50,
      filters: {
        orderBy: 'AC_RATE',
        sortOrder: 'DESCENDING',
        difficulty: 'EASY',
        status: 'NOT_STARTED',
        tags: ['array'],
      },
    },
  })

  let config = {
    method: 'post',
    url: 'https://leetcode.cn/graphql/',
    headers: {
      accept: '*/*',
      'accept-language': 'en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7',
      'content-type': 'application/json',
      cookie: process.env.COOKIE ?? '',
    },
    data: data,
  }

  return axios(config)
    .then((response) => {
      fs.writeFileSync(quickFileName, JSON.stringify(response.data, null, 2))
    })
    .catch((error) => {
      console.log(error)
    })
}

export const quickFileName = path.join(__dirname, './quick-dive-in.json')

export async function genList() {
  const problem_list: any[] = fs
    .readJSONSync(quickFileName)
    .data.problemsetQuestionList.questions.filter(
      (item: { frontendQuestionId: string }) => {
        return (
          !item.frontendQuestionId.startsWith('LCP') &&
          !item.frontendQuestionId.startsWith('LCS')
        )
      },
    )
    .map((item: { frontendQuestionId: string }) =>
      parseInt(item.frontendQuestionId),
    )

  console.log(`problem_list ${problem_list.length}`)
  const r = await Problem.findAll({
    where: {
      frontend_id: {
        [Op.in]: problem_list,
      },
    },
  })

  await generatePool({
    problems: r,
  })

  return problem_list
}
