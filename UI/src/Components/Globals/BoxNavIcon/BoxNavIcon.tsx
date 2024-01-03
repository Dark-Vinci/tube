import { JSX } from 'react';

interface BoxNavIconProps {
  readonly name: string;
  readonly icon: JSX.Element;
}

export function BoxNavIcon({ icon, name }: BoxNavIconProps): JSX.Element {
  return (
    <div>
      <div>{icon}</div>
      <div>{name}</div>
    </div>
  );
}
