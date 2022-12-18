import { useEffect, useRef } from 'react'
import { useAppSelector } from '../../hooks/useAppSelector'
import { useAppDispatch } from '../../hooks/useDispatch'
import { Fee } from '../../models/fee.model'
import {
  changeMode,
  createFee,
  deleteFee,
  fetchFees,
} from '../../redux/reducers/fees.reducer'
import { CreateDialog } from './components/CreateDialog/CreateDialog'
import { FeesView } from './FeesView'
import { confirmDialog, ConfirmDialog } from 'primereact/confirmdialog'
import { Toast, ToastProps } from 'primereact/toast'
import { Messages } from 'primereact/messages'

export const FeesContainer = () => {
  const dispatch = useAppDispatch()
  const fees = useAppSelector((state) => state.fees.fees)
  const mode = useAppSelector((state) => state.fees.mode)
  const errorSelector = useAppSelector((state) => state.fees.error)

  const errorToast = useRef<Toast>(null)

  useEffect(() => {
    if (mode === 'list') {
      dispatch(fetchFees())
    }
  }, [mode])

  useEffect(() => {
    if (errorSelector) {
      errorToast.current?.show({
        severity: 'error',
        summary: errorSelector.summary,
        detail: errorSelector.detail,
      })
    }
  }, [errorSelector])

  const createFeeHandler = (fee: Fee) => {
    dispatch(createFee(fee))
  }

  const openDeleteDialog = (id: number) => {
    dispatch(changeMode('delete'))
    confirmDialog({
      message: 'Datensatz wirklich löschen?',
      header: 'Datensatz löschen',
      accept: () => {
        dispatch(deleteFee({ id }))
      },
      reject: () => {
        dispatch(changeMode('list'))
      },
    })
  }

  return (
    <div className="h-full">
      <Toast ref={errorToast} />
      <ConfirmDialog></ConfirmDialog>
      <CreateDialog
        show={mode === 'create'}
        fees={fees}
        onCreate={(fee) => createFeeHandler(fee)}
      ></CreateDialog>

      <FeesView fees={fees} onDelete={(id) => openDeleteDialog(id)} />
    </div>
  )
}
