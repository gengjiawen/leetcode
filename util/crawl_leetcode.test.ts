import { saveRecord } from './crawl_leetcode'

let execute_test = false

test('sync records', async () => {
  if (execute_test) {
    await saveRecord()
  }
})
