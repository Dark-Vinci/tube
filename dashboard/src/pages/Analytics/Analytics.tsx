import style from './Analytics.module.scss';
import { IfElse } from '@component';

export function Analytics(): JSX.Element {
  return (
    <div className={style.page}>
      <IfElse
        condition={false}
        ifElem={<div> if condition1</div>}
        elseElem={<div>else element</div>}
      />

      <strong>This is analytic page</strong>
    </div>
  );
}
