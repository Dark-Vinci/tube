import axios, { AxiosInstance } from 'axios';

export function createAxios(): AxiosInstance {
  const instance = axios.create({
    baseURL: 'localhost:3030',
    timeout: 1000,
    headers: { 'X-Custom-Header': 'foobar' },
  });

  instance.interceptors.request.use(
    function (config) {
      return config;
    },
    function (error) {
      console.log({
        type: 'CLIENT ERROR',
        error: JSON.stringify(error),
      });
      return Promise.reject(error);
    },
  );

  instance.interceptors.response.use(
    function (response) {
      return response;
    },
    function (error) {
      console.log({
        type: 'SERVER ERROR',
        error: JSON.stringify(error),
      });
      return Promise.reject(error);
    },
  );

  return instance;
}
