import { useDispatch } from 'react-redux'
import { AppDispatch } from '../redux/app.store'

export const useAppDispatch: () => AppDispatch = useDispatch
