import "./init.js"
import { start } from './general.js'
import { fetchSpoken, prevNextEvent, insertAll, addForm } from './spoken.js'
import { compute, insertArray } from './compute.js'

let path = window.location.pathname;

if (path === '/spoken') {
  start('/spoken/count').then(({ ok: max }) => {
    if (!max) {
      throw 'we must have the total available words before fetching all languages'
    }

    conn.spoken.max = parseInt(max)
    if (conn.spoken.on === null) {
      addForm()
    } else {
      fetchSpoken(conn.spoken)
      prevNextEvent(conn, '.Spoken-F-next')
      prevNextEvent(conn, '.Spoken-F-Prev')
      const reset = document.querySelector('.Spoken-T-reset');
      reset.addEventListener('click', () => {
        localStorage.clear()
        location.reload()
      })
    }
  }).catch(e => {
    console.log(e)
  })

} else if (path === '/compute') {
  const BestComplexity = document.querySelector('#C-best')
  const WorstComplexity = document.querySelector('#C-worst')
  const computeNext = document.querySelector('.Compute-F-next')
  const computePrev = document.querySelector('.Compute-F-prev')
  insertArray(BestComplexity, 'best')
  insertArray(WorstComplexity, 'worst')
  compute(computeNext, 'next')
  compute(computePrev, 'prev')
} else if (path === '/spoken/all') {
  insertAll()
}
