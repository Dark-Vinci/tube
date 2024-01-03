import { JSX } from 'react';
import { MdHomeFilled, MdSubscriptions, MdMenu } from 'react-icons/md';
import { SiYoutubeshorts } from 'react-icons/si';
import { TfiLayoutMediaCenterAlt } from 'react-icons/tfi';

import { BoxNavIcon } from '@components';
import style from './Aside.module.scss';

const a = [
  {
    name: 'Home',
    icon: <MdHomeFilled />,
  },

  {
    name: 'Shorts',
    icon: <SiYoutubeshorts />,
  },

  {
    name: 'Subscriptions',
    icon: <MdSubscriptions />,
  },

  {
    name: 'You',
    icon: <TfiLayoutMediaCenterAlt />,
  },
];

export function Aside(): JSX.Element {
  return (
    <div className={style.container}>
      <div className={style.aside_container}>
        <div className={style.menu}>
          <MdMenu />
        </div>

        <div className={style.nav}>
          <div className={style.nav_container}>
            {a.map(({ name, icon }, i) => {
              return (
                <div key={i}>
                  <BoxNavIcon name={name} icon={icon} />
                </div>
              );
            })}
          </div>
        </div>
      </div>
    </div>
  );
}
