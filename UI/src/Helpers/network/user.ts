import { AxiosInstance } from 'axios';

import { createAxios } from './axios';

export class UserRequests {
  public constructor(private readonly axios: AxiosInstance = createAxios('')) {}

  public async signIn(email: string, password: string): Promise<string> {
    console.log({ email, password });

    try {
      const response = await this.axios.post('/login', { email, password });

      return JSON.stringify(response);
    } catch (error: any) {
      throw new Error(error!.message);
    }
  }
}
