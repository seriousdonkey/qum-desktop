export interface Fee {
  id?: number
  baseFee: number
  pricePerUnit: number
  monthlyDiscount: number
  dateStart: string
  dateEnd: string
}
