
import { bubbleSort, onBubbleSort } from './general.js'

test("bubbleSort | best | worst", () => {
  let sorted = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
  let linear = bubbleSort(sorted)
  expect(linear[sorted.length - 1]).toEqual(sorted)

  let unsorted = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
  let square = bubbleSort(unsorted)
  let calculate = unsorted.length * (unsorted.length - 1)
  expect(square[calculate]).toEqual(sorted)
})
test("bubbleSort | Average", () => {
  let sorted = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
  let halfSorted = [0, 1, 2, 3, 4, 9, 8, 7, 6, 5]
  let calculate = halfSorted.length * ((halfSorted.length - 1) / 2)
  let res = bubbleSort(halfSorted)
  expect(res[calculate]).toEqual(sorted)
})

test("bubbleSort | Browser", () => {
  let sorted = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
  let state = {
    list: [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
    count: 0,
    start: 1,
    swapped: false
  }
  let prevCount = null
  while (state.count !== prevCount) {
    prevCount = state.count
    state = onBubbleSort(state)
  }
  expect(state.list).toEqual(sorted)
  expect(state.count).toEqual(sorted.length * (sorted.length - 1))
})

test("bubbleSort | Prev", () => {
  let stack = []
  let state = {
    list: [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
    count: 0,
    start: 1,
    swapped: false
  }

  for (const _ of state.list) {
    stack.push({ ...state })
    state = onBubbleSort(state)
  }
  while (stack.length > 0) {
    let prev = stack.pop()
    expect(prev.count).toEqual(state.count - 1)
    state.count = prev.count
  }
})