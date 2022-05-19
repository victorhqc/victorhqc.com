import { useEffect, useState, ReactNode } from 'react';
import NextImage from 'next/image';
import { isServer } from '@/src/utils/next';
import styles from '@/pageSrc/home/styles.module.css';

export default function RenderBlur({
  blur,
  url,
  children,
}: {
  blur: string;
  url: string;
  children: (state: State) => ReactNode;
}) {
  const [state, setState] = useState<State>({
    imageLoaded: false,
    blurVisible: true,
  });

  useEffect(() => {
    if (isServer()) return;

    const img = new Image();
    img.addEventListener('load', () => {
      setState({ imageLoaded: true, blurVisible: true });
      setTimeout(() => {
        setState({ imageLoaded: true, blurVisible: false });
      }, 500);
    });

    img.src = url;
  }, [url]);

  return (
    <>
      {state.blurVisible ? (
        <NextImage src={blur} layout="fill" className={styles.image__blur} />
      ) : null}
      {children(state)}
    </>
  );
}

type State = {
  imageLoaded: boolean;
  blurVisible: boolean;
};
