import { useEffect, useState, ReactNode } from 'react';
import NextImage from 'next/image';
import useNextBlurHash from 'use-next-blurhash';
import styles from '@/pageSrc/home/styles.module.css';

export default function RenderBlur({
  blurHash,
  url,
  children,
}: {
  blurHash: string;
  url: string;
  children: (isReady: boolean) => ReactNode;
}) {
  const [blur] = useNextBlurHash(blurHash);
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    const img = new Image();
    img.addEventListener('load', () => {
      setIsReady(true);
    });

    img.src = url;
  }, [url]);

  return (
    <>
      {!isReady ? (
        <NextImage src={blur} layout="fill" className={styles.image__blur} />
      ) : null}
      {children(isReady)}
    </>
  );
}
