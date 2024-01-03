import { JSX, useEffect } from 'react';
import { RouterProvider } from 'react-router-dom';

import { Route } from '@router';
import './App.scss';

export function App(): JSX.Element {
  useEffect(() => {
    console.log('JUST LANDED ON THE PAGE');
  }, []);

  return (
    <div className="App">
      <RouterProvider
        router={Route({})}
        fallbackElement={<div>This service is unable to load</div>}
      />
    </div>
  );
}
