export interface IfElseProps {
  readonly condition: boolean;
  readonly ifElement: React.ReactNode;
  readonly elseElement: React.ReactNode;
}

export function IfElse({
  condition,
  ifElement,
  elseElement,
}: IfElseProps): JSX.Element {
  if (condition) {
    return ifElement as JSX.Element;
  }

  return elseElement as JSX.Element;
}
