import { describe, expect, it, beforeAll, beforeEach } from 'vitest'
import { getNewCheckersRules, CheckersRules } from '../../rules'
import { Color, PawnType } from '@/types'

const initBoard: { [key: string]: [Color, PawnType] } = {
  '1_1': [Color.Empty, PawnType.Empty],
  '1_2': [Color.Black, PawnType.PawnBlack],
  '1_3': [Color.Empty, PawnType.Empty],
  '1_4': [Color.Black, PawnType.PawnBlack],
  '1_5': [Color.Empty, PawnType.Empty],
  '1_6': [Color.Black, PawnType.PawnBlack],
  '1_7': [Color.Empty, PawnType.Empty],
  '1_8': [Color.Black, PawnType.PawnBlack],
  '2_1': [Color.Black, PawnType.PawnBlack],
  '2_2': [Color.Empty, PawnType.Empty],
  '2_3': [Color.Black, PawnType.PawnBlack],
  '2_4': [Color.Empty, PawnType.Empty],
  '2_5': [Color.Black, PawnType.PawnBlack],
  '2_6': [Color.Empty, PawnType.Empty],
  '2_7': [Color.Black, PawnType.PawnBlack],
  '2_8': [Color.Empty, PawnType.Empty],
  '3_1': [Color.Empty, PawnType.Empty],
  '3_2': [Color.Black, PawnType.PawnBlack],
  '3_3': [Color.Empty, PawnType.Empty],
  '3_4': [Color.Black, PawnType.PawnBlack],
  '3_5': [Color.Empty, PawnType.Empty],
  '3_6': [Color.Black, PawnType.PawnBlack],
  '3_7': [Color.Empty, PawnType.Empty],
  '3_8': [Color.Black, PawnType.PawnBlack],
  '4_1': [Color.Empty, PawnType.Empty],
  '4_2': [Color.Empty, PawnType.Empty],
  '4_3': [Color.Empty, PawnType.Empty],
  '4_4': [Color.Empty, PawnType.Empty],
  '4_5': [Color.Empty, PawnType.Empty],
  '4_6': [Color.Empty, PawnType.Empty],
  '4_7': [Color.Empty, PawnType.Empty],
  '4_8': [Color.Empty, PawnType.Empty],
  '5_1': [Color.Empty, PawnType.Empty],
  '5_2': [Color.Empty, PawnType.Empty],
  '5_3': [Color.Empty, PawnType.Empty],
  '5_4': [Color.Empty, PawnType.Empty],
  '5_5': [Color.Empty, PawnType.Empty],
  '5_6': [Color.Empty, PawnType.Empty],
  '5_7': [Color.Empty, PawnType.Empty],
  '5_8': [Color.Empty, PawnType.Empty],
  '6_1': [Color.White, PawnType.PawnWhite],
  '6_2': [Color.Empty, PawnType.Empty],
  '6_3': [Color.Black, PawnType.PawnWhite],
  '6_4': [Color.Empty, PawnType.Empty],
  '6_5': [Color.White, PawnType.PawnWhite],
  '6_6': [Color.Empty, PawnType.Empty],
  '6_7': [Color.White, PawnType.PawnWhite],
  '6_8': [Color.Empty, PawnType.Empty],
  '7_1': [Color.Empty, PawnType.Empty],
  '7_2': [Color.White, PawnType.PawnWhite],
  '7_3': [Color.Empty, PawnType.Empty],
  '7_4': [Color.White, PawnType.PawnWhite],
  '7_5': [Color.Empty, PawnType.Empty],
  '7_6': [Color.White, PawnType.PawnWhite],
  '7_7': [Color.Empty, PawnType.Empty],
  '7_8': [Color.White, PawnType.PawnWhite],
  '8_1': [Color.White, PawnType.PawnWhite],
  '8_2': [Color.Empty, PawnType.Empty],
  '8_3': [Color.White, PawnType.PawnWhite],
  '8_4': [Color.Empty, PawnType.Empty],
  '8_5': [Color.White, PawnType.PawnWhite],
  '8_6': [Color.Empty, PawnType.Empty],
  '8_7': [Color.White, PawnType.PawnWhite],
  '8_8': [Color.Empty, PawnType.Empty]
}

describe('CheckersRules tests', () => {
  let checkersRules: CheckersRules
  describe('CheckersRules test canMove', () => {
    beforeEach(() => {
      checkersRules = getNewCheckersRules()
    })

    it('test canMove rule - can move Black', () => {
      checkersRules.nextTurn()
      expect(checkersRules.canMove(3, 2, 4, 3, initBoard)).to.be.true
    })

    it("test canMove rule - can't move Black first", () => {
      expect(checkersRules.canMove(3, 2, 4, 3, initBoard)).to.be.false
    })

    it('test canMove rule - can move White', () => {
      expect(checkersRules.canMove(6, 1, 5, 2, initBoard)).to.be.true
    })

    it("test canMove rule - can't move ", () => {
      expect(checkersRules.canMove(6, 1, 5, 1, initBoard)).to.be.false
      expect(checkersRules.canMove(6, 1, 4, 1, initBoard)).to.be.false
    })
  })

  describe('CheckersRules test canBeat - Pawn', () => {
    let newBoard: { [key: string]: [Color, PawnType] }

    beforeAll(() => {
      newBoard = Object.assign({}, initBoard)
      newBoard['6_1'] = [Color.Empty, PawnType.Empty]
      newBoard['5_2'] = [Color.White, PawnType.PawnWhite]
      newBoard['3_4'] = [Color.Empty, PawnType.Empty]
      newBoard['4_3'] = [Color.Black, PawnType.PawnBlack]
    })

    beforeEach(() => {
      checkersRules = getNewCheckersRules()
    })

    it('test canBeat rule - can move White', () => {
      expect(checkersRules.canBeat(5, 2, 3, 4, newBoard)).to.be.true
    })

    it('test canBeat rule - can move Black', () => {
      checkersRules.nextTurn()
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
      newBoard['3_4'] = [Color.White, PawnType.PawnWhite]

      expect(checkersRules.canBeat(3, 4, 5, 2, newBoard)).to.be.false
    })
  })

  describe('CheckersRules test canMove - Dame', () => {
    let newBoard: { [key: string]: [Color, PawnType] }

    beforeAll(() => {
      newBoard = {
        '1_1': [Color.Empty, PawnType.Empty],
        '1_2': [Color.Empty, PawnType.Empty],
        '1_3': [Color.Empty, PawnType.Empty],
        '1_4': [Color.Empty, PawnType.Empty],
        '1_5': [Color.Empty, PawnType.Empty],
        '1_6': [Color.Empty, PawnType.Empty],
        '1_7': [Color.Empty, PawnType.Empty],
        '1_8': [Color.Empty, PawnType.Empty],
        '2_1': [Color.Empty, PawnType.Empty],
        '2_2': [Color.Empty, PawnType.Empty],
        '2_3': [Color.Empty, PawnType.Empty],
        '2_4': [Color.Empty, PawnType.Empty],
        '2_5': [Color.Empty, PawnType.Empty],
        '2_6': [Color.Empty, PawnType.Empty],
        '2_7': [Color.Empty, PawnType.Empty],
        '2_8': [Color.Empty, PawnType.Empty],
        '3_1': [Color.Empty, PawnType.Empty],
        '3_2': [Color.Empty, PawnType.Empty],
        '3_3': [Color.Empty, PawnType.Empty],
        '3_4': [Color.Black, PawnType.Dame],
        '3_5': [Color.Empty, PawnType.Empty],
        '3_6': [Color.Empty, PawnType.Empty],
        '3_7': [Color.Empty, PawnType.Empty],
        '3_8': [Color.Empty, PawnType.Empty],
        '4_1': [Color.Empty, PawnType.Empty],
        '4_2': [Color.Empty, PawnType.Empty],
        '4_3': [Color.Empty, PawnType.Empty],
        '4_4': [Color.Empty, PawnType.Empty],
        '4_5': [Color.Black, PawnType.PawnBlack],
        '4_6': [Color.Empty, PawnType.Empty],
        '4_7': [Color.Empty, PawnType.Empty],
        '4_8': [Color.Empty, PawnType.Empty],
        '5_1': [Color.Empty, PawnType.Empty],
        '5_2': [Color.White, PawnType.PawnWhite],
        '5_3': [Color.Empty, PawnType.Empty],
        '5_4': [Color.Empty, PawnType.Empty],
        '5_5': [Color.Empty, PawnType.Empty],
        '5_6': [Color.Empty, PawnType.Empty],
        '5_7': [Color.Empty, PawnType.Empty],
        '5_8': [Color.Empty, PawnType.Empty],
        '6_1': [Color.Empty, PawnType.Empty],
        '6_2': [Color.Empty, PawnType.Empty],
        '6_3': [Color.White, PawnType.Dame],
        '6_4': [Color.Empty, PawnType.Empty],
        '6_5': [Color.Empty, PawnType.Empty],
        '6_6': [Color.Empty, PawnType.Empty],
        '6_7': [Color.Empty, PawnType.Empty],
        '6_8': [Color.Empty, PawnType.Empty],
        '7_1': [Color.Empty, PawnType.Empty],
        '7_2': [Color.Empty, PawnType.Empty],
        '7_3': [Color.Empty, PawnType.Empty],
        '7_4': [Color.Empty, PawnType.Empty],
        '7_5': [Color.Empty, PawnType.Empty],
        '7_6': [Color.Empty, PawnType.Empty],
        '7_7': [Color.Empty, PawnType.Empty],
        '7_8': [Color.Empty, PawnType.Empty],
        '8_1': [Color.Empty, PawnType.Empty],
        '8_2': [Color.Empty, PawnType.Empty],
        '8_3': [Color.Empty, PawnType.Empty],
        '8_4': [Color.Empty, PawnType.Empty],
        '8_5': [Color.Empty, PawnType.Empty],
        '8_6': [Color.Empty, PawnType.Empty],
        '8_7': [Color.White, PawnType.Dame],
        '8_8': [Color.Empty, PawnType.Empty]
      }
    })
    beforeEach(() => {
      checkersRules = getNewCheckersRules()
    })

    it('test canMove rule - can move Black', () => {
      checkersRules.nextTurn()
      expect(checkersRules.canMove(3, 4, 6, 1, newBoard)).to.be.true
    })

    it("test canMove rule - can't move Black first", () => {
      expect(checkersRules.canMove(3, 2, 5, 4, newBoard)).to.be.false
    })

    it('test canMove rule - can move White', () => {
      expect(checkersRules.canMove(8, 7, 2, 1, newBoard)).to.be.true
    })

    it("test canMove rule - can't move - White", () => {
      expect(checkersRules.canMove(8, 7, 6, 7, newBoard)).to.be.false
    })

    it("test canMove rule - can't move - Black", () => {
      checkersRules.nextTurn()
      expect(checkersRules.canMove(3, 4, 5, 4, newBoard)).to.be.false
    })
  })

  describe('CheckersRules test canBeat - Dame', () => {
    let newBoard: { [key: string]: [Color, PawnType] }

    beforeAll(() => {
      newBoard = {
        '1_1': [Color.Empty, PawnType.Empty],
        '1_2': [Color.Empty, PawnType.Empty],
        '1_3': [Color.Empty, PawnType.Empty],
        '1_4': [Color.Empty, PawnType.Empty],
        '1_5': [Color.Empty, PawnType.Empty],
        '1_6': [Color.Empty, PawnType.Empty],
        '1_7': [Color.Empty, PawnType.Empty],
        '1_8': [Color.Empty, PawnType.Empty],
        '2_1': [Color.Empty, PawnType.Empty],
        '2_2': [Color.Empty, PawnType.Empty],
        '2_3': [Color.Empty, PawnType.Empty],
        '2_4': [Color.Empty, PawnType.Empty],
        '2_5': [Color.Empty, PawnType.Empty],
        '2_6': [Color.Empty, PawnType.Empty],
        '2_7': [Color.Empty, PawnType.Empty],
        '2_8': [Color.Empty, PawnType.Empty],
        '3_1': [Color.Empty, PawnType.Empty],
        '3_2': [Color.Empty, PawnType.Empty],
        '3_3': [Color.Empty, PawnType.Empty],
        '3_4': [Color.Black, PawnType.Dame],
        '3_5': [Color.Empty, PawnType.Empty],
        '3_6': [Color.Empty, PawnType.Empty],
        '3_7': [Color.Empty, PawnType.Empty],
        '3_8': [Color.Empty, PawnType.Empty],
        '4_1': [Color.Empty, PawnType.Empty],
        '4_2': [Color.Empty, PawnType.Empty],
        '4_3': [Color.Empty, PawnType.Empty],
        '4_4': [Color.Empty, PawnType.Empty],
        '4_5': [Color.Black, PawnType.PawnBlack],
        '4_6': [Color.Empty, PawnType.Empty],
        '4_7': [Color.Empty, PawnType.Empty],
        '4_8': [Color.Empty, PawnType.Empty],
        '5_1': [Color.Empty, PawnType.Empty],
        '5_2': [Color.White, PawnType.PawnWhite],
        '5_3': [Color.Empty, PawnType.Empty],
        '5_4': [Color.Empty, PawnType.Empty],
        '5_5': [Color.Empty, PawnType.Empty],
        '5_6': [Color.Empty, PawnType.Empty],
        '5_7': [Color.Empty, PawnType.Empty],
        '5_8': [Color.Empty, PawnType.Empty],
        '6_1': [Color.Empty, PawnType.Empty],
        '6_2': [Color.Empty, PawnType.Empty],
        '6_3': [Color.White, PawnType.Dame],
        '6_4': [Color.Empty, PawnType.Empty],
        '6_5': [Color.Empty, PawnType.Empty],
        '6_6': [Color.Empty, PawnType.Empty],
        '6_7': [Color.Empty, PawnType.Empty],
        '6_8': [Color.Empty, PawnType.Empty],
        '7_1': [Color.Empty, PawnType.Empty],
        '7_2': [Color.Empty, PawnType.Empty],
        '7_3': [Color.Empty, PawnType.Empty],
        '7_4': [Color.Empty, PawnType.Empty],
        '7_5': [Color.Empty, PawnType.Empty],
        '7_6': [Color.Empty, PawnType.Empty],
        '7_7': [Color.Empty, PawnType.Empty],
        '7_8': [Color.Empty, PawnType.Empty],
        '8_1': [Color.Empty, PawnType.Empty],
        '8_2': [Color.Empty, PawnType.Empty],
        '8_3': [Color.Empty, PawnType.Empty],
        '8_4': [Color.Empty, PawnType.Empty],
        '8_5': [Color.Empty, PawnType.Empty],
        '8_6': [Color.Empty, PawnType.Empty],
        '8_7': [Color.Empty, PawnType.Empty],
        '8_8': [Color.Empty, PawnType.Empty]
      }
    })

    beforeEach(() => {
      checkersRules = getNewCheckersRules()
    })

    it('test canBeat rule - can move White', () => {
      expect(checkersRules.canBeat(6, 3, 3, 6, newBoard)).to.be.true
    })

    it('test canBeat rule - can move Black', () => {
      checkersRules.nextTurn()
      expect(checkersRules.canBeat(3, 4, 6, 1, newBoard)).to.be.true
    })

    it("test canBeat rule - can't move", () => {
      expect(checkersRules.canBeat(4, 3, 6, 2, newBoard)).to.be.false
      expect(checkersRules.canBeat(4, 3, 5, 1, newBoard)).to.be.false
      expect(checkersRules.canBeat(5, 2, 4, 4, newBoard)).to.be.false
      expect(checkersRules.canBeat(5, 2, 4, 4, newBoard)).to.be.false
    })
  })
})
