import dateFormat from 'dateformat'
import { Button } from 'primereact/button'
import { Column } from 'primereact/column'
import { DataTable } from 'primereact/datatable'
import { useEffect } from 'react'
import { useAppDispatch } from '../../hooks/useDispatch'
import { Fee } from '../../models/fee.model'
import { changeMode, fetchFees } from '../../redux/reducers/fees.reducer'

export interface FeesViewProps {
  fees: Fee[]
  onDelete: (id: number) => void
}

export const FeesView = (props: FeesViewProps) => {
  const dispatch = useAppDispatch()

  useEffect(() => {
    console.log('fees:', props.fees)
  }, [props.fees])

  const header = (
    <div className="flex align-items-center justify-content-between gap-2">
      <h2>Gebühren</h2>
      <div className="flex align-items-center gap-2">
        <Button
          icon="fa-solid fa-rotate-right"
          onClick={() => dispatch(fetchFees())}
        ></Button>
      </div>
    </div>
  )

  const footer = (
    <div className="flex flex-row align-items-center justify-content-end">
      <Button
        icon="fa-solid fa-plus"
        label="Neu"
        onClick={() => dispatch(changeMode('create'))}
      />
    </div>
  )

  const currencyFormat = (value: number) => {
    return value.toLocaleString('de-DE', {
      style: 'currency',
      currency: 'EUR',
    })
  }

  const baseFeeBodyTemplate = (fee: Fee) => {
    return currencyFormat(fee.baseFee)
  }

  const pricePerUnitBodyTemplate = (fee: Fee) => {
    return currencyFormat(fee.pricePerUnit)
  }

  const monthlyDiscountBodyTemplate = (fee: Fee) => {
    return currencyFormat(fee.monthlyDiscount)
  }

  const dateStartBodyTemplate = (fee: Fee) => {
    return dateFormat(fee.dateStart, 'dd.mm.yyyy')
  }

  const dateEndBodyTemplate = (fee: Fee) => {
    return dateFormat(fee.dateEnd, 'dd.mm.yyyy')
  }

  const actionsBodyTemplate = (fee: Fee) => {
    return (
      <div>
        <Button
          icon="fa-solid fa-trash"
          className="p-button-text p-button-rounded p-button-danger"
          onClick={() => props.onDelete(fee.id != undefined ? fee.id : 0)}
        ></Button>
      </div>
    )
  }

  return (
    <DataTable
      value={props.fees}
      header={header}
      footer={footer}
      scrollable={true}
      scrollHeight="flex"
    >
      <Column header="Basispreis (Monat)" body={baseFeeBodyTemplate} />
      <Column header="Preis pro kWh" body={pricePerUnitBodyTemplate} />
      <Column
        header="Monatlicher Abschlag"
        body={monthlyDiscountBodyTemplate}
      />
      <Column header="Startdatum" body={dateStartBodyTemplate} />
      <Column header="Enddatum" body={dateEndBodyTemplate} />
      <Column body={actionsBodyTemplate} />
    </DataTable>
  )
}
