import { JSX } from 'react';
import { Outlet } from 'react-router-dom';

import style from './Layout.module.scss';

export function Layout(): JSX.Element {
  return (
    <div className={style.container}>
      <nav></nav>
      <Outlet />
    </div>
  );
}
