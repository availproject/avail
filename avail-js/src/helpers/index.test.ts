import { isValidAddress, formatNumberToBalance } from "."
import { BN } from "@polkadot/util"

test("It returns true for a valid address", () => {
  const alice = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
  expect(isValidAddress(alice)).toBe(true)
})

test("It returns false for a wrong string", () => {
  const wrong = "Hello from the other side"
  expect(isValidAddress(wrong)).toBe(false)
})


describe('formatNumberToBalance', () => {
  it('should correctly format number with default 18 decimals', () => {
    const value = 123;
    const expected = new BN('' + 123 * 10**18);
    expect(formatNumberToBalance(value)).toEqual(expected);
  });

  it('should correctly format a decimal number with default 18 decimals', () => {
    const value = 123.456;
    const expected = new BN('' + 123.456 * 10**18);
    expect(formatNumberToBalance(value)).toEqual(expected);
  });

  it('should correctly format a string number with decimals', () => {
    const value = '123.456';
    const expected = new BN('123456000000');
    expect(formatNumberToBalance(value, 9)).toEqual(expected);
  });

  it('should throw an error for large integer values', () => {
    const value = 10 ** 11;
    expect(() => formatNumberToBalance(value)).toThrow('For big representation of number, please use a string instead of a number');
  });

});
