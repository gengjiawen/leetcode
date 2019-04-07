test('tt', () => {
  let a = 1
  let b = null
  const c = {a, b}
  console.log(c)
  const d = JSON.stringify(c)
  console.log(d)
})
