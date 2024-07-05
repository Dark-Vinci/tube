import { IfElse } from '@component';

export default function Comments(): JSX.Element {
  return (
    <div>
      <IfElse
        condition={false}
        ifElem={<div> if condition1</div>}
        elseElem={<div>else element</div>}
      />

      <strong>This is analytic page</strong>
    </div>
  );
}
