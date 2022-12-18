import './Sidenav.scss'
import {
  faCalculator,
  faFileSignature,
  faHome,
  faMagnifyingGlassChart,
} from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { Link } from 'react-router-dom'

export const Sidenav = () => {
  return (
    <div className="sidebar">
      <div className="logo-details flex-align-items-center justify-content-center gap-1">
        <FontAwesomeIcon icon={faMagnifyingGlassChart} size="2x" />
        <span className="logo-name">Qum</span>
      </div>

      <ul className="nav-links">
        <li>
          <Link to={`dashboard`} className="link">
            <FontAwesomeIcon icon={faHome} />
            <span>Dashboard</span>
          </Link>
        </li>

        <li>
          <h4>Strom</h4>
          <ul className="submenu">
            <li>
              <Link to={`meter-readings`} className="link">
                <FontAwesomeIcon icon={faCalculator} />
                <span>Zählerstand</span>
              </Link>
            </li>
            <li>
              <Link to={`fees`} className="link">
                <FontAwesomeIcon icon={faFileSignature} />
                <span>Gebühren</span>
              </Link>
            </li>
          </ul>
        </li>
      </ul>
    </div>
  )
}
