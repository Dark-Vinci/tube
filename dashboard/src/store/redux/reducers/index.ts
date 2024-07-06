import { combineReducers } from 'redux';
import {
  useSelector as useReduxSelector,
  TypedUseSelectorHook,
} from 'react-redux';

import { authReducer, AuthState } from './auth';

export interface AppState {
  auth: AuthState;
}

export const rootReducer = combineReducers<any>({
  auth: authReducer,
});

export const useSelector: TypedUseSelectorHook<ReturnType<typeof rootReducer>> =
  useReduxSelector;
