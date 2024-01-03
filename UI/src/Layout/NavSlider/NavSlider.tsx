import { JSX } from 'react';

import style from './NavSlider.module.scss';

export function NavSlider(): JSX.Element {
  return (
    <div className={style.container}>
      <div className={style.blurr_bg}></div>
      <div className={style.content}>
        <div className={style.content_container}>
          <div className={style.top}>
            <div>MENU</div>
            <div>LOGO</div>
          </div>

          <div>
            <div className={style.nav}></div>

            <div className={style.you}></div>
          </div>
        </div>
      </div>
    </div>
  );
}
