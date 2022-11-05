import { saveRecord } from './crawl_leetcode'

test('sync records', async () => {
  if (process.env.LOCAL) {
    await saveRecord()
  }
})
