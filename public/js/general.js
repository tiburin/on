
export const start = async path => {
  let response = await fetch(window.location.origin + path)
  if (response.status === 200) {
    return { ok: await response.text() }
  } else {
    return { error: "not found" }
  }
}

export const parse = content => {
  let [rank, word, sentence] = content.split(',').map(value => value.trim())
  rank = parseInt(rank)
  return [rank, word || "none", sentence || 'none']
}


// WORST N^2
// N * (N - 1)
// BEST  N
// Not save the largest number of the last iteration  
// [8, 7, 6, 5, 4, 3, 2, 1, 0, (9)]

export const bubbleSort = list => {
  let count = 0
  let start = 1
  let swapped = false

  while (start < list.length) {
    let left = list[start - 1]
    let right = list[start]

    if (left > right) {
      list[start - 1] = right
      list[start] = left
      swapped = true
    }

    start++
    count++

    if (start === list.length && swapped) {
      start = 1
      swapped = false
    }
  }
  return { [count]: list }
}
export const onBubbleSort = state => {
  while (state.start < state.list.length) {
    let left = state.list[state.start - 1]
    let right = state.list[state.start]

    if (left > right) {
      state.list[state.start - 1] = right
      state.list[state.start] = left
      state.swapped = true
    }

    state.start++
    state.count++

    if (state.start === state.list.length && state.swapped) {
      state.start = 1
      state.swapped = false
    }

    return state
  }
  return state
}

////// SPINNER

let count = 0
const getNode = position => document.querySelector(`.T-${position}`)
const startSpinner = () => {
  let counter = count += 1
  getNode(count).style.opacity = '0'

  if (count === 1) {
    getNode(3).style.opacity = '1'
  } else if (count === 2) {
    getNode(1).style.opacity = '1'
  } else {
    getNode(2).style.opacity = '1'
  }

  if (counter == 3) {
    count = 0
  }
}

if (document.querySelector('.Spin')) {
  setInterval(startSpinner, 1000)
}
/////////////