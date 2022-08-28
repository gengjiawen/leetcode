import { extract_rust_solution, underline } from './utils'
import { Problem, sequelize } from './crawl_leetcode'
import { generatePool, JSconfig } from './pools_utils'

test('test underline filename', () => {
  const a = underline('minimum-swaps-to-group-all-1s-together')
  expect(a).toBe('minimum_swaps_to_group_all_1s_together')
})

test('extract rust solutions', () => {
  const str = `impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {

    }
}`
  const r = extract_rust_solution(str)
  expect(r).toMatchSnapshot()
})

test('reference test', () => {
  function append_str(o: any) {
    o.str += '__append'
  }

  const o = { str: 'hi' }
  append_str(o)
  console.log(o)
})

test('leetcode statistics', async () => {
  const r = await sequelize.query(
    `select difficulty, count(*) from leetcode group by difficulty;`,
  )
  console.log(`easy medium hard`, r[0])
  const paidCount = await sequelize.query(
    `select count(*) from leetcode where paidOnly == 1;`,
  )
  console.log(`paid count`, paidCount[0])
})

test('generate special', async () => {
  const r = await Problem.findAll({
    where: {
      frontend_id: 141,
    },
  })

  await generatePool({
    problems: r,
    config: JSconfig,
  })
})
