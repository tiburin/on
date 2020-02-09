import { start, parse } from './general.js'


test("fetch one", async () => {
  let rank = 2

  const response = await start(`/spoken/english/${rank}`);
  let key = response["ok"]
  expect(key).toBeTruthy()
  expect(key.includes(rank)).toBeTruthy()
})

test("fetch value not found", async () => {
  let rank = -2
  const response = await start(`/spoken/english/${rank}`);
  let key = response["error"]
  expect(key).toBeTruthy()
  expect(key.includes(rank)).toBeFalsy()
})

test("parse file", async () => {
  const rank = "2"
  const { "ok": content } = await start(`/spoken/english/${rank}`);
  let MainList = parse(content)
  for (let list of MainList) {
    expect(list.length).toBe(3)
  }
})