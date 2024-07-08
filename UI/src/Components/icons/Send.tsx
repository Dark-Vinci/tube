import { View, Text, StyleSheet } from 'react-native';
import React from 'react';

interface sendProp {
  isActive: boolean;
}

export function Send({}: sendProp): JSX.Element {
  return (
    <View style={style.container}>
      <Text>SEND</Text>
    </View>
  );
}

const style = StyleSheet.create({
  container: {},
});
