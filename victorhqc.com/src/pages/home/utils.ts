export function yearsDiff(date: Date) {
  const msDiff = Date.now() - date.getTime();
  const diff = new Date(msDiff);
  return Math.abs(diff.getUTCFullYear() - 1970);
}
