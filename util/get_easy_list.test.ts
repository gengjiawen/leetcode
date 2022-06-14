import { genList, getEasyList, quickFileName } from './get_easy_list'
import * as fs from 'fs'

test('test get easy list', async () => {
  const r = await getEasyList()
  console.log(r)
})

test('test gen list', async () => {
  if (!fs.existsSync(quickFileName)) {
    return
  }
  const r = await genList()
  console.log(r)
})
