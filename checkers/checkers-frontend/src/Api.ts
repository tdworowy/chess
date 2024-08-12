import type { Color, PawnType, Player } from './types'
import axios from 'axios'

type moveJson = {
  player: Player
  board_state: { [key: string]: { pawn_color: string; pawn_type: string } }
}

export class Api {
  //TODO make it consistant
  //TODO fix Same Origin Policy
  //TODO return correct json

  static prepareJson(color: Player, boardState: { [key: string]: [Color, PawnType] }): moveJson {
    const colorMap = {
      '': 'Empty',
      Dark: 'Black',
      Light: 'White'
    }
    const pawnMap = {
      '': 'Empty',
      pawnDark: 'Pawn',
      pawnLight: 'Pawn',
      dame: 'Dame'
    }

    const new_board_state = <{ [key: string]: { pawn_color: string; pawn_type: string } }>{}

    for (const [key, value] of Object.entries(boardState)) {
      new_board_state[key] = { pawn_color: colorMap[value[0]], pawn_type: pawnMap[value[1]] }
    }
    const json = { player: color, board_state: new_board_state }
    return json
  }

  static makeRandomMove(jsonObj: moveJson) {
    axios
      .post('http://localhost:8080/make_radnom_move', JSON.stringify(jsonObj), {
        headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': '*' }
      })
      .then(function (response) {
        console.log(response)
      })
      .catch(function (error) {
        console.log(error)
      })
  }
}
