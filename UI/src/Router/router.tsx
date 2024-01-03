import { createBrowserRouter } from 'react-router-dom';

import { Layout } from '@layout';

interface RouteProps {}

export function Route(state: RouteProps): any {
  console.log({ state });
  return createBrowserRouter(
    [
      {
        path: '/auth',
        Component: Layout,
      },
      {
        path: '/',
        Component: Layout,
        children: [
          {
            path: '/tl',
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
    ],
    { basename: '/' },
  );
}
