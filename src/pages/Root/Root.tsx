import './Root.scss'
import { Link, Outlet } from 'react-router-dom'
import { Sidenav } from '../../components/Sidenav/Sidenav'

export const Root = () => {
  return (
    <div className="h-screen flex flex-row">
      <Sidenav />

      <div className="content">
        <Outlet />
      </div>
    </div>
  )
}
