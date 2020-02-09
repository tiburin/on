import { onBubbleSort } from './general.js'

export const putTemplate = ({ list, start, swapped, count }) => {
  let computeStart = document.querySelector('.Compute-H-start')
  let computeSwapped = document.querySelector('.Compute-H-swap')
  let computeList = document.querySelector('.Compute-list')
  let left = document.querySelector('.Compute-F-left')
  let right = document.querySelector('.Compute-F-right')
  const html = `[
 <span class="Compute-L-n Compute-L-${list[0]}">${list[0]},</span>
 <span class="Compute-L-n Compute-L-${list[1]}">${list[1]},</span>
 <span class="Compute-L-n Compute-L-${list[2]}">${list[2]},</span>
 <span class="Compute-L-n Compute-L-${list[3]}">${list[3]},</span>
 <span class="Compute-L-n Compute-L-${list[4]}">${list[4]},</span>
 <span class="Compute-L-n Compute-L-${list[5]}">${list[5]},</span>
 <span class="Compute-L-n Compute-L-${list[6]}">${list[6]},</span>
 <span class="Compute-L-n Compute-L-${list[7]}">${list[7]},</span>
 <span class="Compute-L-n Compute-L-${list[8]}">${list[8]},</span>
 <span class="Compute-L-n Compute-L-${list[9]}">${list[9]}</span>
 ]`

  computeList.innerHTML = html
  left.innerHTML = `${list[start - 1]}`
  right.innerHTML = `${list[start]}`
  computeSwapped.innerHTML = `Swapped: <span class="Compute-H-Swapped-${swapped} Compute-N-S-value">${swapped}</span>`
  computeStart.innerText = count
  document.querySelector(`.Compute-L-${list[start - 1]}`).classList.add('Compute-N-left', 'Compute-N-S-value')
  document.querySelector(`.Compute-L-${list[start]}`).classList.add('Compute-N-right', 'Compute-N-S-value')

}

const putIntoState = () => {
  let nodes = document.querySelectorAll('.Compute-L-n')
  let list = conn.compute.state.list
  for (let node of nodes) {
    list.push(parseInt(node.textContent))
  }
  conn.compute.state.list = list
}
window.onload = putIntoState


export const insertArray = (event, type) => {
  event.addEventListener('click', e => {
    e.stopPropagation()
    if (type === 'best') {
      conn.compute = conn.compute.new()
      conn.compute.state.list = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
      conn.compute.state.start = 1

      putTemplate(conn.compute.state)
    } else if (type === 'worst') {
      conn.compute = conn.compute.new()
      conn.compute.state.list = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
      conn.compute.state.start = 1
      putTemplate(conn.compute.state)
    }
  })

}

// Compute
export const compute = (event, type) => {
  event.addEventListener('click', e => {
    e.stopPropagation()
    let state = conn.compute.state
    if (type === 'next') {
      const newState = { ...state }
      newState.list = [...state.list]
      conn.compute.stack.push(newState)
      state = onBubbleSort(state)
      if (state.start < state.list.length) {
        putTemplate(state)
      } else {
        // TODO: remove alert | reset state
        alert('list sorted')
      }
    } else if (type === 'prev') {
      if (conn.compute.stack.length > 0) {
        let p = conn.compute.stack.pop()
        conn.compute.state = p
        putTemplate(p)
      }
    }
  })

}