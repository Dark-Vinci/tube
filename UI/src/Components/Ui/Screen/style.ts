import { Platform, StatusBar, StyleSheet } from 'react-native';

export const style = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
    paddingTop:
      Platform.OS === 'android' ? (StatusBar.currentHeight as number) : 0,
  },
});
