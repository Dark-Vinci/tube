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

  scroll: {
    width: '100%',
    height: '100%',
    flex: 1,
    flexGrow: 1,
  },

  scrollContent: {
    justifyContent: 'center',
    alignItems: 'center',
  },
});
