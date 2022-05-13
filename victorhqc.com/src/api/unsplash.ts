import got from 'got';
import { getAPIUrl } from './client';
import { Picture } from './entities';

export async function getRandomPicture() {
  const { picture } = await got(`${getUnsplashAPIUrl()}/picture`, {
    searchParams: { query: '', orientation: '' },
  }).json<{ picture: Picture }>();

  return picture;
}

export function getUnsplashAPIUrl(): string {
  return `${getAPIUrl()}/unsplash`;
}
