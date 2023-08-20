export type boardStateType = (newState: { [key: string]: [Color, PawnType] }) => void

export enum Color {
  Dark = 'Dark',
  Light = 'Light',
  Empty = ''
}

export enum PawnType {
  PawnLight = 'pawnLight',
  PawnDark = 'pawnDark',
  Dame = 'dame',
  Empty = ''
}
