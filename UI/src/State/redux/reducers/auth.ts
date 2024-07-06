import { AuthActionTypes } from '../actionTypes';

export interface AuthState {}

export interface AuthAction {
  type: AuthActionTypes;
  payload: any;
}

const defaultAuthState: AuthState = {};

export function authReducer(
  state: AuthState = defaultAuthState,
  action: AuthAction,
): AuthState {
  switch (action.type) {
    case AuthActionTypes.INITIATE_LOGIN:
      return state;
    default:
      return state;
  }
}
