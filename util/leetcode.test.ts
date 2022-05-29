import { extract_rust_solution, underline } from './utils'

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
