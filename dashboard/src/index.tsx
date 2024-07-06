import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider } from 'react-router-dom';
import {
  legacy_createStore as createStore,
  applyMiddleware,
  compose,
} from 'redux';
import { Provider } from 'react-redux';

import { reportWebVitals } from '@startup';
import { router } from '@router';
import { rootReducer } from 'store';
import './index.scss';

const composeEnhancer = window.__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || compose;
const store = createStore(rootReducer, composeEnhancer(applyMiddleware(thunk)));

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement,
);
root.render(
  <React.StrictMode>
    <Provider store={store}>
      <RouterProvider
        router={router}
        fallbackElement={<div> something went wrong</div>}
      />
    </Provider>
  </React.StrictMode>,
);

reportWebVitals();

declare global {
  interface Window {
    __REDUX_DEVTOOLS_EXTENSION_COMPOSE__: any;
  }
}
