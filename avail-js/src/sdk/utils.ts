import { err, ok, Result } from "neverthrow"

/**
 * Converts a commission percentage to a perbill format.
 *
 * @param {number} value - The commission percentage (0-100).
 * @return {string} The commission value in perbill format.
 * @throws {Error} If the value is not an integer or is out of the 0-100 range.
 */
export function commissionNumberToPerbill(value: number): Result<string, string> {
  if (!Number.isInteger(value)) {
    return err("Commission cannot have decimal place. It needs to be a whole number.")
  }

  if (value < 0 || value > 100) {
    return err("Commission is limited to the following range: 0 - 100. It cannot be less than 0 or more than 100.")
  }

  let commission = value.toString().concat("0000000")
  // For some reason 0 commission is not defined as "0" but as "1".
  if (commission == "00000000") {
    commission = "1"
  }

  return ok(commission)
}
