import {
  createStackNavigator,
  StackNavigationOptions,
} from '@react-navigation/stack';
import React, { View } from 'react-native';
import { NavigationContainer } from '@react-navigation/native';

import { navigationRef } from './util';
import { AppRoute } from './routes';
import { SuperRootStackParamList } from './type';

function Home(): JSX.Element {
  return <View>It is I</View>;
}

const RootStack = createStackNavigator<SuperRootStackParamList>();

export function Navigation(): JSX.Element {
  const navigationOptions: StackNavigationOptions = {
    headerShown: false,
    gestureEnabled: false,
    cardStyle: {},
  };

  return (
    <NavigationContainer ref={navigationRef}>
      <RootStack.Navigator
        initialRouteName={AppRoute.HOME}
        screenOptions={navigationOptions}
      >
        <RootStack.Screen name={AppRoute.HOME} component={Home} />
        <RootStack.Screen name={AppRoute.ANOTHER} component={Home} />
      </RootStack.Navigator>
    </NavigationContainer>
  );
}
