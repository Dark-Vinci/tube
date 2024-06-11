import React from 'react';

export interface IfElseProps {
  readonly condition: boolean;
  readonly ifElem: React.ReactNode;
  readonly elseElem: React.ReactNode;
}

export function IfElse({
  condition,
  elseElem,
  ifElem,
}: IfElseProps): JSX.Element {
  if (condition) {
    return ifElem as JSX.Element;
  }

  return elseElem as JSX.Element;
}
