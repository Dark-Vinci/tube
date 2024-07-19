import React, { JSX } from 'react';
import { View, Text, StyleSheet } from 'react-native';

import { Screen } from '@components';

export function NoConnection(): JSX.Element {
  return (
    <Screen>
      <View style={style.container}>
        <Text>This is the spalsh screen</Text>
      </View>
    </Screen>
  );
}

const style = StyleSheet.create({
  container: {},
});
