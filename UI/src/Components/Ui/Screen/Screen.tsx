import React from 'react';
import { SafeAreaView, ScrollView } from 'react-native';

import { ScreenProps } from './type';
import { style } from './style';

export function Screen({ children }: ScreenProps): JSX.Element {
  return (
    <SafeAreaView style={style.container}>
      <ScrollView> {children} </ScrollView>
    </SafeAreaView>
  );
}
