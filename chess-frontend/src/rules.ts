class CheckersRules {
  canMove(startX: number, startY: number, endX: number, endY: number): boolean {
    const _canMove = Math.abs(startX - endX) === 1 && Math.abs(startY - endY) === 1
    return _canMove
  }
}

export const checkersRules = new CheckersRules()
