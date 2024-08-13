import { Color, PawnType } from '@/types'

export const pieceCondition = (x: number, y: number) => {
  return (y % 2 === 0 && x % 2 !== 0) || (y % 2 !== 0 && x % 2 === 0)
}

export const pieceColorCondiion = (x: number, y: number): [Color, PawnType] =>
  [1, 2, 3].includes(x) && pieceCondition(x, y)
    ? [Color.Black, PawnType.PawnBlack]
    : [6, 7, 8].includes(x) && pieceCondition(x, y)
    ? [Color.White, PawnType.PawnWhite]
    : [Color.Empty, PawnType.Empty]
