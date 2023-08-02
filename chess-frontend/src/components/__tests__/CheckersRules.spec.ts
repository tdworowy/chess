import { describe, expect, it, beforeAll } from 'vitest'
import { checkersRules } from '../../rules'
import { Color } from '@/types'

const initBoard = {
  '1_1': '',
  '1_2': Color.Dark,
  '1_3': '',
  '1_4': Color.Dark,
  '1_5': '',
  '1_6': Color.Dark,
  '1_7': '',
  '1_8': Color.Dark,
  '2_1': Color.Dark,
  '2_2': '',
  '2_3': Color.Dark,
  '2_4': '',
  '2_5': Color.Dark,
  '2_6': '',
  '2_7': Color.Dark,
  '2_8': '',
  '3_1': '',
  '3_2': Color.Dark,
  '3_3': '',
  '3_4': Color.Dark,
  '3_5': '',
  '3_6': Color.Dark,
  '3_7': '',
  '3_8': Color.Dark,
  '4_1': '',
  '4_2': '',
  '4_3': '',
  '4_4': '',
  '4_5': '',
  '4_6': '',
  '4_7': '',
  '4_8': '',
  '5_1': '',
  '5_2': '',
  '5_3': '',
  '5_4': '',
  '5_5': '',
  '5_6': '',
  '5_7': '',
  '5_8': '',
  '6_1': Color.Light,
  '6_2': '',
  '6_3': Color.Light,
  '6_4': '',
  '6_5': Color.Light,
  '6_6': '',
  '6_7': Color.Light,
  '6_8': '',
  '7_1': '',
  '7_2': Color.Light,
  '7_3': '',
  '7_4': Color.Light,
  '7_5': '',
  '7_6': Color.Light,
  '7_7': '',
  '7_8': Color.Light,
  '8_1': Color.Light,
  '8_2': '',
  '8_3': Color.Light,
  '8_4': '',
  '8_5': Color.Light,
  '8_6': '',
  '8_7': Color.Light,
  '8_8': ''
}

describe('CheckersRules tests', () => {
  describe('CheckersRules test canMove', () => {
    it('test canMove rule - can move dark', () => {
      expect(checkersRules.canMove(3, 2, 4, 3, initBoard)).to.be.true
    })

    it('test canMove rule - can move light', () => {
      expect(checkersRules.canMove(6, 1, 5, 2, initBoard)).to.be.true
    })

    it("test canMove rule - can't move", () => {
      expect(checkersRules.canMove(6, 1, 5, 1, initBoard)).to.be.false
      expect(checkersRules.canMove(6, 1, 4, 1, initBoard)).to.be.false
    })
  })

  describe('CheckersRules test canBeat', () => {
    let newBoard: { [key: string]: string }

    beforeAll(() => {
      newBoard = Object.assign({}, initBoard)
      newBoard['6_1'] = ''
      newBoard['5_2'] = Color.Light
      newBoard['3_4'] = ''
      newBoard['4_3'] = Color.Dark
    })

    it('test canBeat rule - can move light', () => {
      expect(checkersRules.canBeat(5, 2, 3, 4, newBoard)).to.be.true
    })

    it('test canBeat rule - can move dark', () => {
      expect(checkersRules.canBeat(4, 3, 6, 1, newBoard)).to.be.true
    })

    it("test canBeat rule - can't move", () => {
      expect(checkersRules.canBeat(4, 3, 6, 2, newBoard)).to.be.false
      expect(checkersRules.canBeat(4, 3, 5, 1, newBoard)).to.be.false
      expect(checkersRules.canBeat(5, 2, 4, 4, newBoard)).to.be.false
      expect(checkersRules.canBeat(5, 2, 4, 4, newBoard)).to.be.false
    })

    it("test canBeat rule - can't move - go back", () => {
      newBoard = Object.assign({}, initBoard)
      newBoard['3_4'] = Color.Light

      expect(checkersRules.canBeat(3, 4, 5, 2, newBoard)).to.be.false
    })
  })
})
