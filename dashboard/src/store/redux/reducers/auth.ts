import { AuthActionType } from '../actionTypes';

export interface AuthState {
  authToken: string;
  userId: string;
}

export interface AuthErrorAction {
  type: AuthActionType;
  payload: {
    message: string;
  };
}
export interface AuthSuccessAction {
  type: AuthActionType;
  payload: any;
}

type AuthActions = AuthSuccessAction | AuthErrorAction;

const initialState = {
  userId: '',
  authToken: 'null',
};

export function authReducer(
  state: AuthState = initialState,
  action: AuthActions,
): AuthState {
  switch (action.type) {
    case AuthActionType.Login:
      return state;
    case AuthActionType.Signup:
      return state;
    default:
      return state;
  }
}
