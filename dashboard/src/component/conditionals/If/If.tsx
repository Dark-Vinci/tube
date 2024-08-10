import React, { JSX } from 'react';

export interface IfProps {
  readonly condition: boolean;
  readonly element: React.ReactNode;
}

export function If({ condition, element }: IfProps): JSX.Element {
  if (!condition) {
    return null as unknown as JSX.Element;
  }

  return element as JSX.Element;
}
