import { createBrowserRouter } from 'react-router-dom'
import { Root } from '../pages/Root/Root'

import Dashboard from '../pages/Dashboard'
import Fees from '../pages/Fees'
import MeterReadings from '../pages/MeterReadings'

export const RouterConfig = createBrowserRouter([
  {
    path: '/',
    element: <Root />,
    children: [
      {
        path: 'dashboard',
        element: <Dashboard />,
      },
      {
        path: 'meter-readings',
        element: <MeterReadings />,
      },
      {
        path: 'fees',
        element: <Fees />,
      },
    ],
  },
])
