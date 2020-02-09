class Spoken {
  constructor() {
    this.user = {}
    this.max = null
    this.rank = parseInt(localStorage.getItem('rank')) || 1
    this.position = 0
    this.on = localStorage.getItem('on')
    this.off = localStorage.getItem('off')
    this.languages = []
    this.onContent = null
    this.offContent = null
  }
}

class Compute {
  constructor() {
    this.stack = []
    this.state = {
      list: [],
      count: 0,
      start: 1,
      swapped: false
    }
  }
  new() {
    return new Compute()
  }
}
class Conn {
  constructor() {
    this.spoken = new Spoken()
    this.compute = new Compute()
  }


}
window.conn = new Conn()