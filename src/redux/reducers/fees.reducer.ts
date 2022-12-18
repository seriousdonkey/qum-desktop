import { createAsyncThunk, createSlice, PayloadAction } from '@reduxjs/toolkit'
import { invoke } from '@tauri-apps/api'
import { Fee } from '../../models/fee.model'

interface State {
  fees: Fee[]
  mode: 'list' | 'create' | 'delete'
  error?: {
    summary: string
    detail?: string
  }
}

const initialState: State = {
  fees: [],
  mode: 'list',
}

export const fetchFees = createAsyncThunk('get_fees_list', async () => {
  const fees = await invoke<Fee[]>('get_fees_list')
  return fees
})

export const createFee = createAsyncThunk('create_fee', async (action: Fee) => {
  const createdFee: Fee = await invoke<Fee>('create_fee', {
    params: action,
  })
  return createdFee
})

export const deleteFee = createAsyncThunk(
  'delete_fee',
  async (action: { id: number }) => {
    await invoke('delete_fee', { id: action.id })
  }
)

export const feesSlice = createSlice({
  name: 'fees',
  initialState,
  reducers: {
    changeMode: (
      state,
      action: PayloadAction<'list' | 'create' | 'delete'>
    ) => {
      state.mode = action.payload
    },
  },
  extraReducers(builder) {
    builder
      .addCase(fetchFees.fulfilled, (state, action) => {
        state.fees = action.payload
      })
      .addCase(createFee.fulfilled, (state) => {
        state.mode = 'list'
      })
      .addCase(createFee.rejected, (state, action) => {
        state.error = {
          summary: 'Gebührendaten konnten nicht erstellt werden',
          detail: action.error.message,
        }
      })

      .addCase(deleteFee.fulfilled, (state) => {
        state.mode = 'list'
      })
      .addCase(deleteFee.rejected, (state, action) => {
        state.error = {
          summary: 'Gebührendaten konnte nicht gelöscht werden',
          detail: action.error.message,
        }
      })
  },
})

export const { changeMode } = feesSlice.actions
export default feesSlice.reducer
