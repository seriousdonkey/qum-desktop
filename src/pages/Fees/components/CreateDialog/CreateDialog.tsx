import { Button } from 'primereact/button'
import { Dialog } from 'primereact/dialog'
import { Calendar, CalendarValueType } from 'primereact/calendar'
import { InputNumber } from 'primereact/inputnumber'
import { Messages } from 'primereact/messages'
import { useAppDispatch } from '../../../../hooks/useDispatch'
import { changeMode } from '../../../../redux/reducers/fees.reducer'
import React, { useEffect, useRef, useState } from 'react'
import { Fee } from '../../../../models/fee.model'
import { parseISO, isAfter, isBefore } from 'date-fns'

interface CreateDialogProps {
  show: boolean
  fees: Fee[]
  onCreate: (fee: Fee) => void
}

export const CreateDialog = (props: CreateDialogProps) => {
  const [baseFee, setBaseFee] = useState<number | null>()
  const [pricePerUnit, setPricePerUnit] = useState<number | null>()
  const [monthlyDiscount, setMonthlyDiscount] = useState<number | null>()
  const [dateRange, setDateRange] = useState<CalendarValueType>()
  const errorMessages = useRef<Messages>(null)
  const dispatch = useAppDispatch()

  useEffect(() => {
    const dates = dateRange as Date[]
    if (dates && dates[0] && dates[1]) {
      const startDate: Date = dates[0]
      const endDate: Date = dates[1]

      const found = props.fees.find((fee) => {
        const feeStartDate = parseISO(fee.dateStart)
        const feeEndDate = parseISO(fee.dateEnd)
        const validateStartDate =
          isAfter(startDate, feeStartDate) && isBefore(startDate, feeEndDate)
        const validateEndDate =
          isAfter(endDate, feeStartDate) && isBefore(endDate, feeEndDate)

        return validateStartDate || validateEndDate
      })

      console.log('found:', found)
      if (found) {
        errorMessages.current?.replace([
          {
            severity: 'error',
            summary:
              'Für den ausgewählten Zeitraum ist bereits ein Datensatz vorhanden',
            sticky: true,
          },
        ])
      } else {
        errorMessages.current?.clear()
      }
    }
  }, [props.fees, dateRange])

  const clearForm = () => {
    setBaseFee(null)
    setPricePerUnit(null)
    setMonthlyDiscount(null)
    setDateRange(null)
  }

  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault()
    const dates = dateRange as Date[]
    const startDate = dates[0].toISOString()
    const endDate = dates[1].toISOString()
    const fee: Fee = {
      baseFee: baseFee!,
      pricePerUnit: pricePerUnit!,
      monthlyDiscount: monthlyDiscount!,
      dateStart: startDate,
      dateEnd: endDate,
    }
    clearForm()
    props.onCreate(fee)
  }

  return (
    <Dialog
      visible={props.show}
      showHeader={false}
      onHide={() => console.log('hide')}
    >
      <div className="flex flex-row align-items-center justify-content-between ml-1 my-3">
        <div className="flex flex-row align-items-center">
          <i className="fa-solid fa-file-signature fa-2x"></i>
          <h2 className="ml-3">Neuer Datensatz</h2>
        </div>
        <i
          className="fa-solid fa-close cursor-pointer"
          onClick={() => {
            dispatch(changeMode('list'))
            clearForm()
          }}
        ></i>
      </div>

      <Messages ref={errorMessages}></Messages>

      <form onSubmit={handleSubmit}>
        <div className="flex flex-row gap-4">
          <div className="flex flex-column gap-4 justify-content-start">
            <div className="p-inputgroup">
              <InputNumber
                value={baseFee}
                mode="decimal"
                minFractionDigits={2}
                maxFractionDigits={2}
                placeholder="Basispreis (Monat)"
                onValueChange={(e) => setBaseFee(e.value)}
              ></InputNumber>
              <span className="p-inputgroup-addon">€</span>
            </div>

            <div className="p-inputgroup">
              <InputNumber
                value={pricePerUnit}
                mode="decimal"
                minFractionDigits={2}
                maxFractionDigits={2}
                placeholder="Preis pro kWh"
                onValueChange={(e) => setPricePerUnit(e.value)}
              ></InputNumber>
              <span className="p-inputgroup-addon">€</span>
            </div>

            <div className="p-inputgroup">
              <InputNumber
                value={monthlyDiscount}
                mode="decimal"
                minFractionDigits={2}
                maxFractionDigits={2}
                placeholder="Monatlicher Abschlag"
                onValueChange={(e) => setMonthlyDiscount(e.value)}
              ></InputNumber>
              <span className="p-inputgroup-addon">€</span>
            </div>
          </div>

          <div>
            <Calendar
              inline
              selectionMode="range"
              value={dateRange}
              onChange={(e) => setDateRange(e.value)}
            ></Calendar>
          </div>
        </div>

        <div className="mt-3 flex flex-row w-full align-items-center justify-content-end">
          <Button type="submit" label="Speichern" icon="fa-solid fa-save" />
        </div>
      </form>
    </Dialog>
  )
}
