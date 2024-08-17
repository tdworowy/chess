import { Color, PawnType, type Player } from './types'
import axios from 'axios'

type moveJson = {
  player: Player
  board_state: { [key: string]: { pawn_color: string; pawn_type: string } }
}

type responseJson = { [key: string]: { pawn_color: string; pawn_type: string } }

export class Api {
  static prepareJson(color: Player, boardState: { [key: string]: [Color, PawnType] }): moveJson {
    const pawnMap = {
      PawnBlack: 'Pawn',
      PawnWhite: 'Pawn',
      Empty: 'Empty',
      Dame: 'Dame'
    }

    const new_board_state = <responseJson>{}

    for (const [key, value] of Object.entries(boardState)) {
      new_board_state[key] = { pawn_color: value[0], pawn_type: pawnMap[value[1]] }
    }
    const json = { player: color, board_state: new_board_state }
    return json
  }

  static parseJson(_responseJson: responseJson): { [key: string]: [Color, PawnType] } {
    const colorMap = {
      Black: Color.Black,
      White: Color.White,
      Empty: Color.Empty
    }

    const newBoardState = <{ [key: string]: [Color, PawnType] }>{}
    for (const [key, value] of Object.entries(_responseJson)) {
      const pawn_type =
        value.pawn_color === Color.Black
          ? PawnType.PawnBlack
          : value.pawn_color === Color.White
          ? PawnType.PawnWhite
          : PawnType.Empty

      newBoardState[key] = [colorMap[value.pawn_color as keyof typeof colorMap], pawn_type]
    }
    return newBoardState
  }

  static makeRandomMove(color: Player, boardState: { [key: string]: [Color, PawnType] }) {
    const jsonObj = Api.prepareJson(color, boardState)
    return axios
      .post('http://localhost:8080/make_radnom_move', JSON.stringify(jsonObj), {
        headers: { 'Content-Type': 'application/json' }
      })
      .then((response) => {
        //console.log(response.data)
        return Api.parseJson(JSON.parse(response.data).board_state)
      })
      .catch((error) => {
        console.log(error)
      })
  }

  static healtCheck() {
    return axios
      .get('http://localhost:8080/healthcheck')
      .then((response) => {
        console.log(response)
        return response.status
      })
      .catch((error) => {
        console.log(error)
      })
  }
}
