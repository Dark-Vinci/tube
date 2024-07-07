import { combineReducers } from 'redux';
import {
  useSelector as useReduxSelector,
  TypedUseSelectorHook,
} from 'react-redux';

import { authReducer } from './auth';

export const rootReducer = combineReducers({
  auth: authReducer,
  bath: authReducer,
});

export type RootState = ReturnType<typeof rootReducer>;



export const useSelector: TypedUseSelectorHook<RootState> = useReduxSelector;
