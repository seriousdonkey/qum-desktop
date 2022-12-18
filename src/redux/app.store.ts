import { configureStore } from '@reduxjs/toolkit'
import feesReducer from './reducers/fees.reducer'

export const store = configureStore({
  reducer: {
    fees: feesReducer,
  },
})

export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch
