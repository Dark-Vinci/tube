import { View, Text, StyleSheet } from 'react-native';
import React from 'react';

interface letterProfileProp {
  letter: string;
}

export function LetterProfile({ letter }: letterProfileProp): JSX.Element {
  return (
    <View style={style.container}>
      <Text>{letter.toUpperCase()}</Text>
    </View>
  );
}

const style = StyleSheet.create({
  container: {},
});
