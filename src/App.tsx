import React from 'react'
import { RouterProvider } from 'react-router-dom'
import 'primereact/resources/themes/lara-light-indigo/theme.css' //theme
import 'primereact/resources/primereact.min.css' //core css
import 'primeicons/primeicons.css' //icons
import 'primeflex/primeflex.min.css'
import './App.scss'
import { RouterConfig } from './navigation/RouterConfig'
import { Provider } from 'react-redux'
import { store } from './redux/app.store'

function App() {
  return (
    <Provider store={store}>
      <RouterProvider router={RouterConfig}></RouterProvider>
    </Provider>
  )
}

export default App
