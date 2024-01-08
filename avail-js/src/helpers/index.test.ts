import { isValidAddress } from "."

test("It returns true for a valid address", () => {
  const alice = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
  expect(isValidAddress(alice)).toBe(true)
})

test("It returns false for a wrong string", () => {
  const wrong = "Hello from the other side"
  expect(isValidAddress(wrong)).toBe(false)
})
