import { createBrowserRouter } from 'react-router-dom';

import { Analytics } from '@pages';

export const router = createBrowserRouter(
  [
    {
      path: '/',
      element: <Analytics />,
      // loader: rootLoader,
      children: [
        {
          path: 'team',
          element: <div>meme</div>,
          // loader: teamLoader,
        },

        {
          path: 'about',
          async lazy() {
            const About = await import('../pages/Comments/Comments');
            return { Component: About } as any;
          },
        },

        {
          path: 'dashboard',
          async lazy() {
            const Dashboard = await import('../pages/Comments/Comments');
            return { Component: Dashboard } as any;
          },
        },
      ],
    },

    {
      path: '*',
      element: <div> 404 page </div>,
    },
  ],
  {},
);
