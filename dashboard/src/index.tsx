import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider } from 'react-router-dom';

import { reportWebVitals } from '@startup';
import { router } from '@router';
import './index.scss';

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement,
);
root.render(
  <React.StrictMode>
    <RouterProvider
      router={router}
      fallbackElement={<div> something went wrong</div>}
    />
  </React.StrictMode>,
);

reportWebVitals();
