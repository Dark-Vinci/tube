import { createBrowserRouter, Outlet } from 'react-router-dom';
import { JSX } from 'react';

// notification

export const router = createBrowserRouter([
  {
    path: '/',
    Component: Layout,
    children: [
      {
        path: '/tl',
        Component: Layout,
      },
      {
        path: '/auth',
        Component: Layout,
      },
      {
        path: '/search-result',
        Component: Layout,
      },
      {
        path: '/short/:id',
        Component: Layout,
      },
      {
        path: '/watch',
        Component: Layout,
      },
      {
        path: '/subscriptions',
        Component: Layout,
      },

      {
        path: '/you',
        Component: Layout,
      },

      {
        path: '/playlist',
        Component: Layout,
      },

      {
        path: '/:channel',
        Component: Layout,
        children: [
          {
            path: '/featured',
            Component: Layout,
          },

          {
            path: '/videos',
            Component: Layout,
          },

          {
            path: '/playlist',
            Component: Layout,
          },

          {
            path: '/community',
            Component: Layout,
          },

          {
            path: '/search',
            Component: Layout,
          },
        ],
      },
    ],
  },
]);

function Layout(): JSX.Element {
  return (
    <div>
      <nav></nav>
      <Outlet />
    </div>
  );
}
