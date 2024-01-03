import { CSSProperties, JSX } from 'react';
import { IconType } from 'react-icons';

import style from './Button.module.scss';

interface ButtonProps {
  style: CSSProperties;
  icon?: IconType;
  title: string;
}

export function Button({
  title,
  style: aStyle,
  icon,
}: ButtonProps): JSX.Element {
  return (
    <button className={style.container} style={aStyle}>
      {`${icon ? icon : ''} ${title}`}
    </button>
  );
}
