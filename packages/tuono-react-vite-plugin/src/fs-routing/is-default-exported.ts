export default function isDefaultExported(source: string): boolean {
  const regex = new RegExp('export default')
  return regex.test(source)
}
