import { AuthActionType } from '../actionTypes';


export const initiateLogin = (): object => {
  return {
    type: AuthActionType.Login,
  };
};
