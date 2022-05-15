import type { UAParser } from 'ua-parser-js';

export function yearsDiff(date: Date) {
  const msDiff = Date.now() - date.getTime();
  const diff = new Date(msDiff);
  return Math.abs(diff.getUTCFullYear() - 1970);
}

export function isMobile(parser: UAParser) {
  if (parser.getDevice().type && parser.getDevice().type !== 'tablet') {
    return true;
  }

  return false;
}
