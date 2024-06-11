import React, { ReactNode } from 'react';
import { SafeAreaView, ScrollView } from 'react-native';

import { style } from './style';

export interface ScreenProps {
  readonly children: ReactNode;
}

export function Screen({ children }: ScreenProps): JSX.Element {
  return (
    <SafeAreaView style={style.container}>
      <ScrollView
        style={style.scroll}
        contentContainerStyle={style.scrollContent}
      >
        {' '}
        {children}{' '}
      </ScrollView>
    </SafeAreaView>
  );
}
