import { describe, expect, it } from 'vitest'
import { Api } from '../Api'
import { Color, PawnType, Player } from '../types'

describe('Api tests', () => {
  it('test parseJson with Dame', () => {
    const responseJson = {
      '1_2': { pawn_color: 'White', pawn_type: 'Dame' },
      '8_1': { pawn_color: 'Black', pawn_type: 'Dame' },
      '3_4': { pawn_color: 'White', pawn_type: 'Pawn' },
      '4_5': { pawn_color: 'Black', pawn_type: 'Pawn' },
      '5_6': { pawn_color: 'Empty', pawn_type: 'Empty' }
    }

    const parsedState = Api.parseJson(responseJson)

    expect(parsedState['1_2']).toEqual([Color.White, PawnType.Dame])
    expect(parsedState['8_1']).toEqual([Color.Black, PawnType.Dame])
    expect(parsedState['3_4']).toEqual([Color.White, PawnType.PawnWhite])
    expect(parsedState['4_5']).toEqual([Color.Black, PawnType.PawnBlack])
    expect(parsedState['5_6']).toEqual([Color.Empty, PawnType.Empty])
  })

  it('test prepareJson with Dame', () => {
    const boardState: { [key: string]: [Color, PawnType] } = {
      '1_2': [Color.White, PawnType.Dame],
      '8_1': [Color.Black, PawnType.Dame],
      '3_4': [Color.White, PawnType.PawnWhite],
      '4_5': [Color.Black, PawnType.PawnBlack],
      '5_6': [Color.Empty, PawnType.Empty]
    }

    const json = Api.prepareJson(Player.White, boardState)

    expect(json.board_state['1_2']).toEqual({ pawn_color: 'White', pawn_type: 'Dame' })
    expect(json.board_state['8_1']).toEqual({ pawn_color: 'Black', pawn_type: 'Dame' })
    expect(json.board_state['3_4']).toEqual({ pawn_color: 'White', pawn_type: 'Pawn' })
    expect(json.board_state['4_5']).toEqual({ pawn_color: 'Black', pawn_type: 'Pawn' })
    expect(json.board_state['5_6']).toEqual({ pawn_color: 'Empty', pawn_type: 'Empty' })
  })
})
