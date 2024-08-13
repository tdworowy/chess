export type boardStateType = (newState: { [key: string]: [Color, PawnType] }) => void

export enum Color {
  Black = 'Black',
  White = 'White',
  Empty = 'Empty'
}

export enum PawnType {
  PawnWhite = 'PawnWhite',
  PawnBlack = 'PawnBlack',
  Dame = 'Dame',
  Empty = 'Empty'
}

export enum Player {
  Black = 'Black',
  White = 'White'
}
