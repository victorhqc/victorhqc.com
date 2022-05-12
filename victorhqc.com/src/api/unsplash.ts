import got from 'got';
import { getAPIUrl } from './client';
import { Picture } from './entities';

export async function getRandomPicture() {
  console.log(
    '`${getUnsplashAPIUrl()}/picture`',
    `${getUnsplashAPIUrl()}/picture`
  );
  const response = await got(`${getUnsplashAPIUrl()}/picture`, {
    searchParams: { query: '', orientation: '' },
  }).json<Picture>();

  console.log('response', response);
}

export function getUnsplashAPIUrl(): string {
  return `${getAPIUrl()}/unsplash`;
}
