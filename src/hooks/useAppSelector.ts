import { TypedUseSelectorHook, useSelector } from 'react-redux'
import { RootState } from '../redux/app.store'

export const useAppSelector: TypedUseSelectorHook<RootState> = useSelector
