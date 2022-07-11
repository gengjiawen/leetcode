import { saveRecord } from './crawl_leetcode'

let execute_test = true

test('sync records', async () => {
  if (execute_test) {
    await saveRecord()
  }
})
