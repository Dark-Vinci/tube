import { SafeAreaView, Text } from 'react-native';
import React, { JSX } from 'react';

export default function App(): JSX.Element {
  return (
    <SafeAreaView
      style={{
        flex: 1,
        backgroundColor: '#fff',
        alignItems: 'center',
        justifyContent: 'center',
      }}
    >
      <Text>Open up App.tsx to start working on your app!</Text>
    </SafeAreaView>
  );
}
