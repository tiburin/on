
import { parse, start } from './general.js'

let mas = list => {
  let result = ''
  for (const val of list) {
    if (val === 'english') {
      result += `<option selected> ${val} </option>`
    } else {
      result += `<option> ${val} </option>`
    }
  }
  return result
}

let createForm = (id, msg1, msg2, opts) => {
  let data = `
  <form id="${id}">
  <h2>${msg1}</h2>
  <h2>${msg2}</h2>
  <select class="mas">
  ${opts}
  </select>
  <button>Next</button>
  </form>
`
  document.querySelector(".Spoken").innerHTML = data
}

let putForm = (list, lang) => {
  if (lang === 'on') {
    createForm('Lang-on', 'Native language!', 'Idioma nativo!', mas(list))
  } else if (lang === 'off') {
    createForm('Lang-off', 'Learning?', 'Aprendiendo?', mas(list))
  }
}

export const addForm = async () => {
  let { ok: languages } = await start('/spoken/languages')
  let list = languages.split('\n').filter(n => n !== "")

  putForm(list, 'on')
  let getForm = document.querySelector('#Lang-on')
  getForm.addEventListener('submit', param => {
    param.preventDefault()
    let d = document.querySelector('.mas')
    localStorage.setItem('on', d.value)
    list = list.filter(n => n !== d.value)
    putForm(list, 'off')
    let getForm2 = document.querySelector('#Lang-off')
    getForm2.addEventListener('submit', param => {
      param.preventDefault()
      let d = document.querySelector('.mas')
      localStorage.setItem('off', d.value)
      location.reload()
    })
  })
}

const putTotal = () => {
  let query = document.querySelector('.Spoken-T-total')
  query.innerHTML = `TOTAL:  <span>${window.conn.spoken.max}</span>`
}

const putLanguage = (content, classRank, classWord, classSentence) => {
  let rank = document.querySelector(classRank)
  let left = document.querySelector(classWord)
  let right = document.querySelector(classSentence)
  rank.textContent = content[0]
  left.innerHTML = `<strong>${content[1]}:</strong>`
  right.textContent = content[2]
  localStorage.setItem('rank', content[0])
  putTotal()
}

const putOnOff = (onContent, offContent) => {
  putLanguage(onContent, '.Spoken-F-rank', '.Spoken-L-B-word', '.Spoken-L-B-sentence')
  putLanguage(offContent, '.Spoken-F-rank', '.Spoken-L-B-palabra', '.Spoken-L-B-oracion')
}
export const fetchSpoken = async spoken => {
  let { ok: on } = await start(`/spoken/${spoken.on}/${spoken.rank}`)
  let { ok: off } = await start(`/spoken/${spoken.off}/${spoken.rank}`)
  let onContent = parse(on)
  let offContent = parse(off)
  spoken.onContent = onContent
  spoken.offContent = offContent
  putOnOff(onContent, offContent)
}



export const prevNextEvent = ({ spoken }, className) => {
  document.querySelector(className).addEventListener('click', e => {
    e.stopPropagation()
    let current = document.querySelector('.Spoken-F-rank').innerText
    if (className === '.Spoken-F-next' && current < spoken.max) {
      spoken.rank += 1
      fetchSpoken(conn.spoken)
    } else if (className === '.Spoken-F-Prev' && current > 1) {
      spoken.rank -= 1
      fetchSpoken(conn.spoken)
    }
  })
}


export const insertAll = async () => {
  let section = document.createElement('section')
  let bodyContent = document.querySelector('.Spoken-all')
  let { ok: one } = await start('/spoken/english')
  let { ok: two } = await start('/spoken/espanol')
  let first = one.trim().split('\n')
  let second = two.trim().split('\n')
  let count = 0

  while (count < first.length && count < second.length) {
    let newNode = document.createElement('div')

    let english = parse(first[count])
    let espanol = parse(second[count])

    newNode.innerHTML = `
    <div class="Spoken-all" draggable="true">
    <span>
      <strong> ${english[0]}</strong>: ${english[1]} ${english[2]}
    </span>
    <span>
      <strong> ${espanol[0]}</strong>: ${espanol[1]} ${espanol[2]}
    </span>
    </div>
  `
    section.appendChild(newNode)
    count++
  }
  bodyContent.append(section)
}
