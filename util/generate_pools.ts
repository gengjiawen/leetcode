import { generatePool, Rust } from './pools_utils'

let test = false
process.argv.includes('--test') && (test = true)
generatePool({ config: Rust, test: test })
  .then(() => {
    console.log('generate pools finished')
  })
  .catch(console.error)
