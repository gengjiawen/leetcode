export function underline(s: string) {
  return s
    .split(' ')
    .map((a) => a.toLowerCase())
    .join('_')
    .replaceAll('?', '')
    .replaceAll('-', '_')
}

export function extract_rust_solution(str: string) {
  const regex = /pub fn .*?}/gms
  if (!str.startsWith('impl Solution')) {
    return str
  }
  const r = str.match(regex)
  if (r) {
    return r[0]
  }
  return str
}
