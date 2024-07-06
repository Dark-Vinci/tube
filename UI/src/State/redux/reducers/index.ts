import { combineReducers } from 'redux';
import {
  useSelector as useReduxSelector,
  TypedUseSelectorHook,
} from 'react-redux';

import { authReducer, AuthState } from './auth';

interface AppState {
  auth: AuthState;
}

export const rootReducer = combineReducers<AppState>({
  auth: authReducer,
});

export type RootState = ReturnType<typeof rootReducer>;

export const useSelector: TypedUseSelectorHook<RootState> = useReduxSelector;
