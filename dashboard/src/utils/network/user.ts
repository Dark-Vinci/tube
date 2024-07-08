import { AxiosInstance } from 'axios';

import { createAxios } from './axios';

export class UserRequests {
  public constructor(private readonly axios: AxiosInstance = createAxios('')) {}

  public async signIn<T>(email: string, password: string): Promise<T> {
    console.log({ email, password });

    try {
      const response = await this.axios.post('/login', { email, password });

      return response as T;
    } catch (error: any) {
      throw new Error(error!.message);
    }
  }
}
