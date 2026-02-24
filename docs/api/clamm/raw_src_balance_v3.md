# Balancer V3 Monorepo - Flattened Solidity Contracts

This file contains all flattened Solidity contracts from the Balancer V3 monorepo.

Generated on: $(date)
Total contracts: $total

---


## ./flattened/pkg/pool-cow/contracts/CowPoolFactory.sol

```solidity
// flattened/pkg/pool-cow/contracts/CowPoolFactory.sol

```


## ./pkg/solidity-utils/contracts/math/WeightedMath.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/solidity-utils/contracts/math/LogExpMath.sol

// solhint-disable

/**
 * @dev Exponentiation and logarithm functions for 18 decimal fixed point numbers (both base and exponent/argument).
 *
 * Exponentiation and logarithm with arbitrary bases (x^y and log_x(y)) are implemented by conversion to natural
 * exponentiation and logarithm (where the base is Euler's number).
 *
 * All math operations are unchecked in order to save gas.
 *
 * @author Fernando Martinelli - @fernandomartinelli
 * @author Sergio Yuhjtman     - @sergioyuhjtman
 * @author Daniel Fernandez    - @dmf7z
 */
library LogExpMath {
    /// @notice This error is thrown when a base is not within an acceptable range.
    error BaseOutOfBounds();

    /// @notice This error is thrown when a exponent is not within an acceptable range.
    error ExponentOutOfBounds();

    /// @notice This error is thrown when the exponent * ln(base) is not within an acceptable range.
    error ProductOutOfBounds();

    /// @notice This error is thrown when an exponent used in the exp function is not within an acceptable range.
    error InvalidExponent();

    /// @notice This error is thrown when a variable or result is not within the acceptable bounds defined in the function.
    error OutOfBounds();

    // All fixed point multiplications and divisions are inlined. This means we need to divide by ONE when multiplying
    // two numbers, and multiply by ONE when dividing them.

    // All arguments and return values are 18 decimal fixed point numbers.
    int256 constant ONE_18 = 1e18;

    // Internally, intermediate values are computed with higher precision as 20 decimal fixed point numbers, and in the
    // case of ln36, 36 decimals.
    int256 constant ONE_20 = 1e20;
    int256 constant ONE_36 = 1e36;

    // The domain of natural exponentiation is bound by the word size and number of decimals used.
    //
    // Because internally the result will be stored using 20 decimals, the largest possible result is
    // (2^255 - 1) / 10^20, which makes the largest exponent ln((2^255 - 1) / 10^20) = 130.700829182905140221.
    // The smallest possible result is 10^(-18), which makes largest negative argument
    // ln(10^(-18)) = -41.446531673892822312.
    // We use 130.0 and -41.0 to have some safety margin.
    int256 constant MAX_NATURAL_EXPONENT = 130e18;
    int256 constant MIN_NATURAL_EXPONENT = -41e18;

    // Bounds for ln_36's argument. Both ln(0.9) and ln(1.1) can be represented with 36 decimal places in a fixed point
    // 256 bit integer.
    int256 constant LN_36_LOWER_BOUND = ONE_18 - 1e17;
    int256 constant LN_36_UPPER_BOUND = ONE_18 + 1e17;

    uint256 constant MILD_EXPONENT_BOUND = 2 ** 254 / uint256(ONE_20);

    // 18 decimal constants
    int256 constant x0 = 128000000000000000000; // 2ˆ7
    int256 constant a0 = 38877084059945950922200000000000000000000000000000000000; // eˆ(x0) (no decimals)
    int256 constant x1 = 64000000000000000000; // 2ˆ6
    int256 constant a1 = 6235149080811616882910000000; // eˆ(x1) (no decimals)

    // 20 decimal constants
    int256 constant x2 = 3200000000000000000000; // 2ˆ5
    int256 constant a2 = 7896296018268069516100000000000000; // eˆ(x2)
    int256 constant x3 = 1600000000000000000000; // 2ˆ4
    int256 constant a3 = 888611052050787263676000000; // eˆ(x3)
    int256 constant x4 = 800000000000000000000; // 2ˆ3
    int256 constant a4 = 298095798704172827474000; // eˆ(x4)
    int256 constant x5 = 400000000000000000000; // 2ˆ2
    int256 constant a5 = 5459815003314423907810; // eˆ(x5)
    int256 constant x6 = 200000000000000000000; // 2ˆ1
    int256 constant a6 = 738905609893065022723; // eˆ(x6)
    int256 constant x7 = 100000000000000000000; // 2ˆ0
    int256 constant a7 = 271828182845904523536; // eˆ(x7)
    int256 constant x8 = 50000000000000000000; // 2ˆ-1
    int256 constant a8 = 164872127070012814685; // eˆ(x8)
    int256 constant x9 = 25000000000000000000; // 2ˆ-2
    int256 constant a9 = 128402541668774148407; // eˆ(x9)
    int256 constant x10 = 12500000000000000000; // 2ˆ-3
    int256 constant a10 = 113314845306682631683; // eˆ(x10)
    int256 constant x11 = 6250000000000000000; // 2ˆ-4
    int256 constant a11 = 106449445891785942956; // eˆ(x11)

    /**
     * @dev Exponentiation (x^y) with unsigned 18 decimal fixed point base and exponent.
     *
     * Reverts if ln(x) * y is smaller than `MIN_NATURAL_EXPONENT`, or larger than `MAX_NATURAL_EXPONENT`.
     */
    function pow(uint256 x, uint256 y) internal pure returns (uint256) {
        if (y == 0) {
            // We solve the 0^0 indetermination by making it equal one.
            return uint256(ONE_18);
        }

        if (x == 0) {
            return 0;
        }

        // Instead of computing x^y directly, we instead rely on the properties of logarithms and exponentiation to
        // arrive at that result. In particular, exp(ln(x)) = x, and ln(x^y) = y * ln(x). This means
        // x^y = exp(y * ln(x)).

        // The ln function takes a signed value, so we need to make sure x fits in the signed 256 bit range.
        if (x >> 255 != 0) {
            revert BaseOutOfBounds();
        }
        int256 x_int256 = int256(x);

        // We will compute y * ln(x) in a single step. Depending on the value of x, we can either use ln or ln_36. In
        // both cases, we leave the division by ONE_18 (due to fixed point multiplication) to the end.

        // This prevents y * ln(x) from overflowing, and at the same time guarantees y fits in the signed 256 bit range.
        if (y >= MILD_EXPONENT_BOUND) {
            revert ExponentOutOfBounds();
        }
        int256 y_int256 = int256(y);

        int256 logxTimes_y;
        unchecked {
            if (LN_36_LOWER_BOUND < x_int256 && x_int256 < LN_36_UPPER_BOUND) {
                int256 ln_36_x = _ln_36(x_int256);

                // ln_36_x has 36 decimal places, so multiplying by y_int256 isn't as straightforward, since we can't just
                // bring y_int256 to 36 decimal places, as it might overflow. Instead, we perform two 18 decimal
                // multiplications and add the results: one with the first 18 decimals of ln_36_x, and one with the
                // (downscaled) last 18 decimals.
                logxTimes_y = ((ln_36_x / ONE_18) * y_int256 + ((ln_36_x % ONE_18) * y_int256) / ONE_18);
            } else {
                logxTimes_y = _ln(x_int256) * y_int256;
            }
            logxTimes_y /= ONE_18;
        }

        // Finally, we compute exp(y * ln(x)) to arrive at x^y
        if (!(MIN_NATURAL_EXPONENT <= logxTimes_y && logxTimes_y <= MAX_NATURAL_EXPONENT)) {
            revert ProductOutOfBounds();
        }

        return uint256(exp(logxTimes_y));
    }

    /**
     * @dev Natural exponentiation (e^x) with signed 18 decimal fixed point exponent.
     *
     * Reverts if `x` is smaller than MIN_NATURAL_EXPONENT, or larger than `MAX_NATURAL_EXPONENT`.
     */
    function exp(int256 x) internal pure returns (int256) {
        if (!(x >= MIN_NATURAL_EXPONENT && x <= MAX_NATURAL_EXPONENT)) {
            revert InvalidExponent();
        }

        // We avoid using recursion here because zkSync doesn't support it.
        bool negativeExponent = false;

        if (x < 0) {
            // We only handle positive exponents: e^(-x) is computed as 1 / e^x. We can safely make x positive since it
            // fits in the signed 256 bit range (as it is larger than MIN_NATURAL_EXPONENT). In the negative
            // exponent case, compute e^x, then return 1 / result.
            unchecked {
                x = -x;
            }
            negativeExponent = true;
        }

        // First, we use the fact that e^(x+y) = e^x * e^y to decompose x into a sum of powers of two, which we call x_n,
        // where x_n == 2^(7 - n), and e^x_n = a_n has been precomputed. We choose the first x_n, x0, to equal 2^7
        // because all larger powers are larger than MAX_NATURAL_EXPONENT, and therefore not present in the
        // decomposition.
        // At the end of this process we will have the product of all e^x_n = a_n that apply, and the remainder of this
        // decomposition, which will be lower than the smallest x_n.
        // exp(x) = k_0 * a_0 * k_1 * a_1 * ... + k_n * a_n * exp(remainder), where each k_n equals either 0 or 1.
        // We mutate x by subtracting x_n, making it the remainder of the decomposition.

        // The first two a_n (e^(2^7) and e^(2^6)) are too large if stored as 18 decimal numbers, and could cause
        // intermediate overflows. Instead we store them as plain integers, with 0 decimals.
        // Additionally, x0 + x1 is larger than MAX_NATURAL_EXPONENT, which means they will not both be present in the
        // decomposition.

        // For each x_n, we test if that term is present in the decomposition (if x is larger than it), and if so deduct
        // it and compute the accumulated product.

        int256 firstAN;
        unchecked {
            if (x >= x0) {
                x -= x0;
                firstAN = a0;
            } else if (x >= x1) {
                x -= x1;
                firstAN = a1;
            } else {
                firstAN = 1; // One with no decimal places
            }

            // We now transform x into a 20 decimal fixed point number, to have enhanced precision when computing the
            // smaller terms.
            x *= 100;
        }

        // `product` is the accumulated product of all a_n (except a0 and a1), which starts at 20 decimal fixed point
        // one. Recall that fixed point multiplication requires dividing by ONE_20.
        int256 product = ONE_20;

        unchecked {
            if (x >= x2) {
                x -= x2;
                product = (product * a2) / ONE_20;
            }
            if (x >= x3) {
                x -= x3;
                product = (product * a3) / ONE_20;
            }
            if (x >= x4) {
                x -= x4;
                product = (product * a4) / ONE_20;
            }
            if (x >= x5) {
                x -= x5;
                product = (product * a5) / ONE_20;
            }
            if (x >= x6) {
                x -= x6;
                product = (product * a6) / ONE_20;
            }
            if (x >= x7) {
                x -= x7;
                product = (product * a7) / ONE_20;
            }
            if (x >= x8) {
                x -= x8;
                product = (product * a8) / ONE_20;
            }
            if (x >= x9) {
                x -= x9;
                product = (product * a9) / ONE_20;
            }
        }

        // x10 and x11 are unnecessary here since we have high enough precision already.

        // Now we need to compute e^x, where x is small (in particular, it is smaller than x9). We use the Taylor series
        // expansion for e^x: 1 + x + (x^2 / 2!) + (x^3 / 3!) + ... + (x^n / n!).

        int256 seriesSum = ONE_20; // The initial one in the sum, with 20 decimal places.
        int256 term; // Each term in the sum, where the nth term is (x^n / n!).

        // The first term is simply x.
        term = x;
        unchecked {
            seriesSum += term;

            // Each term (x^n / n!) equals the previous one times x, divided by n. Since x is a fixed point number,
            // multiplying by it requires dividing by ONE_20, but dividing by the non-fixed point n values does not.

            term = ((term * x) / ONE_20) / 2;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 3;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 4;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 5;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 6;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 7;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 8;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 9;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 10;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 11;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 12;
            seriesSum += term;

            // 12 Taylor terms are sufficient for 18 decimal precision.

            // We now have the first a_n (with no decimals), and the product of all other a_n present, and the Taylor
            // approximation of the exponentiation of the remainder (both with 20 decimals). All that remains is to multiply
            // all three (one 20 decimal fixed point multiplication, dividing by ONE_20, and one integer multiplication),
            // and then drop two digits to return an 18 decimal value.

            int256 result = (((product * seriesSum) / ONE_20) * firstAN) / 100;

            // We avoid using recursion here because zkSync doesn't support it.
            return negativeExponent ? (ONE_18 * ONE_18) / result : result;
        }
    }

    /// @dev Logarithm (log(arg, base), with signed 18 decimal fixed point base and argument.
    function log(int256 arg, int256 base) internal pure returns (int256) {
        // This performs a simple base change: log(arg, base) = ln(arg) / ln(base).

        // Both logBase and logArg are computed as 36 decimal fixed point numbers, either by using ln_36, or by
        // upscaling.

        int256 logBase;
        unchecked {
            if (LN_36_LOWER_BOUND < base && base < LN_36_UPPER_BOUND) {
                logBase = _ln_36(base);
            } else {
                logBase = _ln(base) * ONE_18;
            }
        }

        int256 logArg;
        unchecked {
            if (LN_36_LOWER_BOUND < arg && arg < LN_36_UPPER_BOUND) {
                logArg = _ln_36(arg);
            } else {
                logArg = _ln(arg) * ONE_18;
            }

            // When dividing, we multiply by ONE_18 to arrive at a result with 18 decimal places
            return (logArg * ONE_18) / logBase;
        }
    }

    /// @dev Natural logarithm (ln(a)) with signed 18 decimal fixed point argument.
    function ln(int256 a) internal pure returns (int256) {
        // The real natural logarithm is not defined for negative numbers or zero.
        if (a <= 0) {
            revert OutOfBounds();
        }
        if (LN_36_LOWER_BOUND < a && a < LN_36_UPPER_BOUND) {
            unchecked {
                return _ln_36(a) / ONE_18;
            }
        } else {
            return _ln(a);
        }
    }

    /// @dev Internal natural logarithm (ln(a)) with signed 18 decimal fixed point argument.
    function _ln(int256 a) private pure returns (int256) {
        // We avoid using recursion here because zkSync doesn't support it.
        bool negativeExponent = false;

        if (a < ONE_18) {
            // Since ln(a^k) = k * ln(a), we can compute ln(a) as ln(a) = ln((1/a)^(-1)) = - ln((1/a)). If a is less
            // than one, 1/a will be greater than one, so in this case we compute ln(1/a) and negate the final result.
            unchecked {
                a = (ONE_18 * ONE_18) / a;
            }
            negativeExponent = true;
        }

        // First, we use the fact that ln^(a * b) = ln(a) + ln(b) to decompose ln(a) into a sum of powers of two, which
        // we call x_n, where x_n == 2^(7 - n), which are the natural logarithm of precomputed quantities a_n (that is,
        // ln(a_n) = x_n). We choose the first x_n, x0, to equal 2^7 because the exponential of all larger powers cannot
        // be represented as 18 fixed point decimal numbers in 256 bits, and are therefore larger than a.
        // At the end of this process we will have the sum of all x_n = ln(a_n) that apply, and the remainder of this
        // decomposition, which will be lower than the smallest a_n.
        // ln(a) = k_0 * x_0 + k_1 * x_1 + ... + k_n * x_n + ln(remainder), where each k_n equals either 0 or 1.
        // We mutate a by subtracting a_n, making it the remainder of the decomposition.

        // For reasons related to how `exp` works, the first two a_n (e^(2^7) and e^(2^6)) are not stored as fixed point
        // numbers with 18 decimals, but instead as plain integers with 0 decimals, so we need to multiply them by
        // ONE_18 to convert them to fixed point.
        // For each a_n, we test if that term is present in the decomposition (if a is larger than it), and if so divide
        // by it and compute the accumulated sum.

        int256 sum = 0;
        unchecked {
            if (a >= a0 * ONE_18) {
                a /= a0; // Integer, not fixed point division
                sum += x0;
            }

            if (a >= a1 * ONE_18) {
                a /= a1; // Integer, not fixed point division
                sum += x1;
            }

            // All other a_n and x_n are stored as 20 digit fixed point numbers, so we convert the sum and a to this format.
            sum *= 100;
            a *= 100;

            // Because further a_n are  20 digit fixed point numbers, we multiply by ONE_20 when dividing by them.

            if (a >= a2) {
                a = (a * ONE_20) / a2;
                sum += x2;
            }

            if (a >= a3) {
                a = (a * ONE_20) / a3;
                sum += x3;
            }

            if (a >= a4) {
                a = (a * ONE_20) / a4;
                sum += x4;
            }

            if (a >= a5) {
                a = (a * ONE_20) / a5;
                sum += x5;
            }

            if (a >= a6) {
                a = (a * ONE_20) / a6;
                sum += x6;
            }

            if (a >= a7) {
                a = (a * ONE_20) / a7;
                sum += x7;
            }

            if (a >= a8) {
                a = (a * ONE_20) / a8;
                sum += x8;
            }

            if (a >= a9) {
                a = (a * ONE_20) / a9;
                sum += x9;
            }

            if (a >= a10) {
                a = (a * ONE_20) / a10;
                sum += x10;
            }

            if (a >= a11) {
                a = (a * ONE_20) / a11;
                sum += x11;
            }
        }

        // a is now a small number (smaller than a_11, which roughly equals 1.06). This means we can use a Taylor series
        // that converges rapidly for values of `a` close to one - the same one used in ln_36.
        // Let z = (a - 1) / (a + 1).
        // ln(a) = 2 * (z + z^3 / 3 + z^5 / 5 + z^7 / 7 + ... + z^(2 * n + 1) / (2 * n + 1))

        // Recall that 20 digit fixed point division requires multiplying by ONE_20, and multiplication requires
        // division by ONE_20.
        unchecked {
            int256 z = ((a - ONE_20) * ONE_20) / (a + ONE_20);
            int256 z_squared = (z * z) / ONE_20;

            // num is the numerator of the series: the z^(2 * n + 1) term
            int256 num = z;

            // seriesSum holds the accumulated sum of each term in the series, starting with the initial z
            int256 seriesSum = num;

            // In each step, the numerator is multiplied by z^2
            num = (num * z_squared) / ONE_20;
            seriesSum += num / 3;

            num = (num * z_squared) / ONE_20;
            seriesSum += num / 5;

            num = (num * z_squared) / ONE_20;
            seriesSum += num / 7;

            num = (num * z_squared) / ONE_20;
            seriesSum += num / 9;

            num = (num * z_squared) / ONE_20;
            seriesSum += num / 11;

            // 6 Taylor terms are sufficient for 36 decimal precision.

            // Finally, we multiply by 2 (non fixed point) to compute ln(remainder)
            seriesSum *= 2;

            // We now have the sum of all x_n present, and the Taylor approximation of the logarithm of the remainder (both
            // with 20 decimals). All that remains is to sum these two, and then drop two digits to return a 18 decimal
            // value.

            int256 result = (sum + seriesSum) / 100;

            // We avoid using recursion here because zkSync doesn't support it.
            return negativeExponent ? -result : result;
        }
    }

    /**
     * @dev Internal high precision (36 decimal places) natural logarithm (ln(x)) with signed 18 decimal fixed point argument,
     * for x close to one.
     *
     * Should only be used if x is between LN_36_LOWER_BOUND and LN_36_UPPER_BOUND.
     */
    function _ln_36(int256 x) private pure returns (int256) {
        // Since ln(1) = 0, a value of x close to one will yield a very small result, which makes using 36 digits
        // worthwhile.

        // First, we transform x to a 36 digit fixed point value.
        unchecked {
            x *= ONE_18;

            // We will use the following Taylor expansion, which converges very rapidly. Let z = (x - 1) / (x + 1).
            // ln(x) = 2 * (z + z^3 / 3 + z^5 / 5 + z^7 / 7 + ... + z^(2 * n + 1) / (2 * n + 1))

            // Recall that 36 digit fixed point division requires multiplying by ONE_36, and multiplication requires
            // division by ONE_36.
            int256 z = ((x - ONE_36) * ONE_36) / (x + ONE_36);
            int256 z_squared = (z * z) / ONE_36;

            // num is the numerator of the series: the z^(2 * n + 1) term
            int256 num = z;

            // seriesSum holds the accumulated sum of each term in the series, starting with the initial z
            int256 seriesSum = num;

            // In each step, the numerator is multiplied by z^2
            num = (num * z_squared) / ONE_36;
            seriesSum += num / 3;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 5;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 7;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 9;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 11;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 13;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 15;

            // 8 Taylor terms are sufficient for 36 decimal precision.

            // All that remains is multiplying by 2 (non fixed point).
            return seriesSum * 2;
        }
    }
}

// pkg/solidity-utils/contracts/math/FixedPoint.sol

/// @notice Support 18-decimal fixed point arithmetic. All Vault calculations use this for high and uniform precision.
library FixedPoint {
    /// @notice Attempted division by zero.
    error ZeroDivision();

    // solhint-disable no-inline-assembly
    // solhint-disable private-vars-leading-underscore

    uint256 internal constant ONE = 1e18; // 18 decimal places
    uint256 internal constant TWO = 2 * ONE;
    uint256 internal constant FOUR = 4 * ONE;
    uint256 internal constant MAX_POW_RELATIVE_ERROR = 10000; // 10^(-14)

    function mulDown(uint256 a, uint256 b) internal pure returns (uint256) {
        // Multiplication overflow protection is provided by Solidity 0.8.x.
        uint256 product = a * b;

        return product / ONE;
    }

    function mulUp(uint256 a, uint256 b) internal pure returns (uint256 result) {
        // Multiplication overflow protection is provided by Solidity 0.8.x.
        uint256 product = a * b;

        // Equivalent to:
        // result = product == 0 ? 0 : ((product - 1) / FixedPoint.ONE) + 1
        assembly ("memory-safe") {
            result := mul(iszero(iszero(product)), add(div(sub(product, 1), ONE), 1))
        }
    }

    function divDown(uint256 a, uint256 b) internal pure returns (uint256) {
        // Solidity 0.8 reverts with a Panic code (0x11) if the multiplication overflows.
        uint256 aInflated = a * ONE;

        // Solidity 0.8 reverts with a "Division by Zero" Panic code (0x12) if b is zero
        return aInflated / b;
    }

    function divUp(uint256 a, uint256 b) internal pure returns (uint256 result) {
        return mulDivUp(a, ONE, b);
    }

    /// @dev Return (a * b) / c, rounding up.
    function mulDivUp(uint256 a, uint256 b, uint256 c) internal pure returns (uint256 result) {
        // This check is required because Yul's `div` doesn't revert on c==0.
        if (c == 0) {
            revert ZeroDivision();
        }

        // Multiple overflow protection is done by Solidity 0.8.x.
        uint256 product = a * b;

        // The traditional divUp formula is:
        // divUp(x, y) := (x + y - 1) / y
        // To avoid intermediate overflow in the addition, we distribute the division and get:
        // divUp(x, y) := (x - 1) / y + 1
        // Note that this requires x != 0, if x == 0 then the result is zero
        //
        // Equivalent to:
        // result = a == 0 ? 0 : (a * b - 1) / c + 1
        assembly ("memory-safe") {
            result := mul(iszero(iszero(product)), add(div(sub(product, 1), c), 1))
        }
    }

    /**
     * @dev Version of divUp when the input is raw (i.e., already "inflated"). For instance,
     * invariant * invariant (36 decimals) vs. invariant.mulDown(invariant) (18 decimal FP).
     * This can occur in calculations with many successive multiplications and divisions, and
     * we want to minimize the number of operations by avoiding unnecessary scaling by ONE.
     */
    function divUpRaw(uint256 a, uint256 b) internal pure returns (uint256 result) {
        // This check is required because Yul's `div` doesn't revert on b==0.
        if (b == 0) {
            revert ZeroDivision();
        }

        // Equivalent to:
        // result = a == 0 ? 0 : 1 + (a - 1) / b
        assembly ("memory-safe") {
            result := mul(iszero(iszero(a)), add(1, div(sub(a, 1), b)))
        }
    }

    /**
     * @dev Returns x^y, assuming both are fixed point numbers, rounding down. The result is guaranteed to not be above
     * the true value (that is, the error function expected - actual is always positive).
     */
    function powDown(uint256 x, uint256 y) internal pure returns (uint256) {
        // Optimize for when y equals 1.0, 2.0 or 4.0, as those are very simple to implement and occur often in 50/50
        // and 80/20 Weighted Pools
        if (y == ONE) {
            return x;
        } else if (y == TWO) {
            return mulDown(x, x);
        } else if (y == FOUR) {
            uint256 square = mulDown(x, x);
            return mulDown(square, square);
        } else {
            uint256 raw = LogExpMath.pow(x, y);
            uint256 maxError = mulUp(raw, MAX_POW_RELATIVE_ERROR) + 1;

            if (raw < maxError) {
                return 0;
            } else {
                unchecked {
                    return raw - maxError;
                }
            }
        }
    }

    /**
     * @dev Returns x^y, assuming both are fixed point numbers, rounding up. The result is guaranteed to not be below
     * the true value (that is, the error function expected - actual is always negative).
     */
    function powUp(uint256 x, uint256 y) internal pure returns (uint256) {
        // Optimize for when y equals 1.0, 2.0 or 4.0, as those are very simple to implement and occur often in 50/50
        // and 80/20 Weighted Pools
        if (y == ONE) {
            return x;
        } else if (y == TWO) {
            return mulUp(x, x);
        } else if (y == FOUR) {
            uint256 square = mulUp(x, x);
            return mulUp(square, square);
        } else {
            uint256 raw = LogExpMath.pow(x, y);
            uint256 maxError = mulUp(raw, MAX_POW_RELATIVE_ERROR) + 1;

            return raw + maxError;
        }
    }

    /**
     * @dev Returns the complement of a value (1 - x), capped to 0 if x is larger than 1.
     *
     * Useful when computing the complement for values with some level of relative error, as it strips this error and
     * prevents intermediate negative values.
     */
    function complement(uint256 x) internal pure returns (uint256 result) {
        // Equivalent to:
        // result = (x < ONE) ? (ONE - x) : 0
        assembly ("memory-safe") {
            result := mul(lt(x, ONE), sub(ONE, x))
        }
    }
}

// pkg/solidity-utils/contracts/math/WeightedMath.sol

/**
 * @notice Implementation of Balancer Weighted Math, essentially unchanged since v1.
 * @dev It is a generalization of the x * y = k constant product formula, accounting for cases with more than two
 * tokens, and weights that are not 50/50.
 *
 * See https://docs.balancer.fi/concepts/explore-available-balancer-pools/weighted-pool/weighted-math.html
 *
 * For security reasons, to help ensure that for all possible "round trip" paths the caller always receives the same
 * or fewer tokens than supplied, we have chosen the rounding direction to favor the protocol in all cases.
 */
library WeightedMath {
    using FixedPoint for uint256;

    /// @notice Address attempted to extract a disproportionate amountOut of tokens from a pool.
    error MaxOutRatio();

    /// @notice Address attempted to add a disproportionate amountIn of tokens to a pool.
    error MaxInRatio();

    /**
     * @notice Error thrown when the calculated invariant is zero, indicating an issue with the invariant calculation.
     * @dev Most commonly, this happens when a token balance is zero.
     */
    error ZeroInvariant();

    // Pool limits that arise from limitations in the fixed point power function. When computing x^y, the valid range
    // of `x` is -41 (ExpMin) to 130 (ExpMax). See `LogExpMath.sol` for the derivation of these values.
    //
    // Invariant calculation:
    // In computing `balance^normalizedWeight`, `log(balance) * normalizedWeight` must fall within the `pow` function
    // bounds described above. Since 0.01 <= normalizedWeight <= 0.99, the balance is constrained to the range between
    // e^(ExpMin) and e^(ExpMax).
    //
    // This corresponds to 10^(-18) < balance < 2^(188.56). Since the maximum balance is 2^(128) - 1, the invariant
    // calculation is unconstrained by the `pow` function limits.
    //
    // It's a different story with `computeBalanceOutGivenInvariant` (inverse invariant):
    // This uses the power function to raise the invariant ratio to the power of 1/weight. Similar to the computation
    // for the invariant, this means the following expression must hold:
    // ExpMin < log(invariantRatio) * 1/weight < ExpMax
    //
    // Given the valid range of weights (i.e., 1 < 1/weight < 100), we have:
    // ExpMin/100 < log(invariantRatio) < ExpMax/100, or e^(-0.41) < invariantRatio < e^(1.3). Numerically, this
    // constrains the invariantRatio to between 0.661 and 3.695. For an added safety margin, we set the limits to
    // 0.7 < invariantRatio < 3.

    // Swap limits: amounts swapped may not be larger than this percentage of the total balance.
    uint256 internal constant _MAX_IN_RATIO = 30e16; // 30%
    uint256 internal constant _MAX_OUT_RATIO = 30e16; // 30%

    // Invariant growth limit: non-proportional add cannot cause the invariant to increase by more than this ratio.
    uint256 internal constant _MAX_INVARIANT_RATIO = 300e16; // 300%
    // Invariant shrink limit: non-proportional remove cannot cause the invariant to decrease by less than this ratio.
    uint256 internal constant _MIN_INVARIANT_RATIO = 70e16; // 70%

    /**
     * @notice Compute the invariant, rounding down.
     * @dev The invariant functions are called by the Vault during various liquidity operations, and require a specific
     * rounding direction in order to ensure safety (i.e., that the final result is always rounded in favor of the
     * protocol. The invariant (i.e., all token balances) must always be greater than 0, or it will revert.
     *
     * @param normalizedWeights The pool token weights, sorted in token registration order
     * @param balances The pool token balances, sorted in token registration order
     * @return invariant The invariant, rounded down
     */
    function computeInvariantDown(
        uint256[] memory normalizedWeights,
        uint256[] memory balances
    ) internal pure returns (uint256 invariant) {
        /**********************************************************************************************
        // invariant               _____                                                             //
        // wi = weight index i      | |      wi                                                      //
        // bi = balance index i     | |  bi ^   = i                                                  //
        // i = invariant                                                                             //
        **********************************************************************************************/

        invariant = FixedPoint.ONE;
        for (uint256 i = 0; i < normalizedWeights.length; ++i) {
            invariant = invariant.mulDown(balances[i].powDown(normalizedWeights[i]));
        }

        if (invariant == 0) {
            revert ZeroInvariant();
        }
    }

    /**
     * @notice Compute the invariant, rounding up.
     * @dev The invariant functions are called by the Vault during various liquidity operations, and require a specific
     * rounding direction in order to ensure safety (i.e., that the final result is always rounded in favor of the
     * protocol. The invariant (i.e., all token balances) must always be greater than 0, or it will revert.
     *
     * @param normalizedWeights The pool token weights, sorted in token registration order
     * @param balances The pool token balances, sorted in token registration order
     * @return invariant The invariant, rounded up
     */
    function computeInvariantUp(
        uint256[] memory normalizedWeights,
        uint256[] memory balances
    ) internal pure returns (uint256 invariant) {
        /**********************************************************************************************
        // invariant               _____                                                             //
        // wi = weight index i      | |      wi                                                      //
        // bi = balance index i     | |  bi ^   = i                                                  //
        // i = invariant                                                                             //
        **********************************************************************************************/

        invariant = FixedPoint.ONE;
        for (uint256 i = 0; i < normalizedWeights.length; ++i) {
            invariant = invariant.mulUp(balances[i].powUp(normalizedWeights[i]));
        }

        if (invariant == 0) {
            revert ZeroInvariant();
        }
    }

    /**
     * @notice Compute a token balance after a liquidity operation, given the current balance and invariant ratio.
     * @dev This is called as part of the "inverse invariant" `computeBalance` calculation.
     * @param currentBalance The current balance of the token
     * @param weight The weight of the token
     * @param invariantRatio The invariant ratio (i.e., new/old; will be > 1 for add; < 1 for remove)
     * @return newBalance The adjusted token balance after the operation
     */
    function computeBalanceOutGivenInvariant(
        uint256 currentBalance,
        uint256 weight,
        uint256 invariantRatio
    ) internal pure returns (uint256 newBalance) {
        /******************************************************************************************
        // calculateBalanceGivenInvariant                                                        //
        // o = balanceOut                                                                        //
        // b = balanceIn                      (1 / w)                                            //
        // w = weight              o = b * i ^                                                   //
        // i = invariantRatio                                                                    //
        ******************************************************************************************/

        // Rounds result up overall, rounding up the two individual steps:
        // - balanceRatio = invariantRatio ^ (1 / weight)
        // - newBalance = balance * balanceRatio
        //
        // Regarding `balanceRatio`, the exponent is always > FP(1), but the invariant ratio can be either greater or
        // lower than FP(1) depending on whether this is solving an `add` or a `remove` operation.
        // - For i > 1, we need to round the exponent up, as i^x is monotonically increasing for i > 1.
        // - For i < 1, we need to round the exponent down, as as i^x is monotonically decreasing for i < 1.

        function(uint256, uint256) internal pure returns (uint256) divUpOrDown = invariantRatio > 1
            ? FixedPoint.divUp
            : FixedPoint.divDown;

        // Calculate by how much the token balance has to increase to match the invariantRatio.
        uint256 balanceRatio = invariantRatio.powUp(divUpOrDown(FixedPoint.ONE, weight));

        return currentBalance.mulUp(balanceRatio);
    }

    /**
     * @notice Compute the `amountOut` of tokenOut in a swap, given the current balances and weights.
     * @param balanceIn The current balance of `tokenIn`
     * @param weightIn  The weight of `tokenIn`
     * @param balanceOut The current balance of `tokenOut`
     * @param weightOut The weight of `tokenOut`
     * @param amountIn The exact amount of `tokenIn` (i.e., the amount given in an ExactIn swap)
     * @return amountOut The calculated amount of `tokenOut` returned in an ExactIn swap
     */
    function computeOutGivenExactIn(
        uint256 balanceIn,
        uint256 weightIn,
        uint256 balanceOut,
        uint256 weightOut,
        uint256 amountIn
    ) internal pure returns (uint256 amountOut) {
        /**********************************************************************************************
        // outGivenExactIn                                                                           //
        // aO = amountOut                                                                            //
        // bO = balanceOut                                                                           //
        // bI = balanceIn              /      /            bI             \    (wI / wO) \           //
        // aI = amountIn    aO = bO * |  1 - | --------------------------  | ^            |          //
        // wI = weightIn               \      \       ( bI + aI )         /              /           //
        // wO = weightOut                                                                            //
        **********************************************************************************************/

        // Amount out, so we round down overall.

        // The multiplication rounds down, and the subtrahend (power) rounds up (so the base rounds up too).
        // Because bI / (bI + aI) <= 1, the exponent rounds down.

        // Cannot exceed maximum in ratio.
        if (amountIn > balanceIn.mulDown(_MAX_IN_RATIO)) {
            revert MaxInRatio();
        }

        uint256 denominator = balanceIn + amountIn;
        uint256 base = balanceIn.divUp(denominator);
        uint256 exponent = weightIn.divDown(weightOut);
        uint256 power = base.powUp(exponent);

        // Because of rounding up, power can be greater than one. Using complement prevents reverts.
        return balanceOut.mulDown(power.complement());
    }

    /**
     * @notice Compute the `amountIn` of tokenIn in a swap, given the current balances and weights.
     * @param balanceIn The current balance of `tokenIn`
     * @param weightIn  The weight of `tokenIn`
     * @param balanceOut The current balance of `tokenOut`
     * @param weightOut The weight of `tokenOut`
     * @param amountOut The exact amount of `tokenOut` (i.e., the amount given in an ExactOut swap)
     * @return amountIn The calculated amount of `tokenIn` returned in an ExactOut swap
     */
    function computeInGivenExactOut(
        uint256 balanceIn,
        uint256 weightIn,
        uint256 balanceOut,
        uint256 weightOut,
        uint256 amountOut
    ) internal pure returns (uint256 amountIn) {
        /**********************************************************************************************
        // inGivenExactOut                                                                           //
        // aO = amountOut                                                                            //
        // bO = balanceOut                                                                           //
        // bI = balanceIn              /  /            bO             \    (wO / wI)      \          //
        // aI = amountIn    aI = bI * |  | --------------------------  | ^            - 1  |         //
        // wI = weightIn               \  \       ( bO - aO )         /                   /          //
        // wO = weightOut                                                                            //
        **********************************************************************************************/

        // Amount in, so we round up overall.

        // The multiplication rounds up, and the power rounds up (so the base rounds up too).
        // Because b0 / (b0 - a0) >= 1, the exponent rounds up.

        // Cannot exceed maximum out ratio.
        if (amountOut > balanceOut.mulDown(_MAX_OUT_RATIO)) {
            revert MaxOutRatio();
        }

        uint256 base = balanceOut.divUp(balanceOut - amountOut);
        uint256 exponent = weightOut.divUp(weightIn);
        uint256 power = base.powUp(exponent);

        // Because the base is larger than one (and the power rounds up), the power should always be larger than one, so
        // the following subtraction should never revert.
        uint256 ratio = power - FixedPoint.ONE;

        return balanceIn.mulUp(ratio);
    }
}

```


## ./pkg/solidity-utils/contracts/math/FixedPoint.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/solidity-utils/contracts/math/LogExpMath.sol

// solhint-disable

/**
 * @dev Exponentiation and logarithm functions for 18 decimal fixed point numbers (both base and exponent/argument).
 *
 * Exponentiation and logarithm with arbitrary bases (x^y and log_x(y)) are implemented by conversion to natural
 * exponentiation and logarithm (where the base is Euler's number).
 *
 * All math operations are unchecked in order to save gas.
 *
 * @author Fernando Martinelli - @fernandomartinelli
 * @author Sergio Yuhjtman     - @sergioyuhjtman
 * @author Daniel Fernandez    - @dmf7z
 */
library LogExpMath {
    /// @notice This error is thrown when a base is not within an acceptable range.
    error BaseOutOfBounds();

    /// @notice This error is thrown when a exponent is not within an acceptable range.
    error ExponentOutOfBounds();

    /// @notice This error is thrown when the exponent * ln(base) is not within an acceptable range.
    error ProductOutOfBounds();

    /// @notice This error is thrown when an exponent used in the exp function is not within an acceptable range.
    error InvalidExponent();

    /// @notice This error is thrown when a variable or result is not within the acceptable bounds defined in the function.
    error OutOfBounds();

    // All fixed point multiplications and divisions are inlined. This means we need to divide by ONE when multiplying
    // two numbers, and multiply by ONE when dividing them.

    // All arguments and return values are 18 decimal fixed point numbers.
    int256 constant ONE_18 = 1e18;

    // Internally, intermediate values are computed with higher precision as 20 decimal fixed point numbers, and in the
    // case of ln36, 36 decimals.
    int256 constant ONE_20 = 1e20;
    int256 constant ONE_36 = 1e36;

    // The domain of natural exponentiation is bound by the word size and number of decimals used.
    //
    // Because internally the result will be stored using 20 decimals, the largest possible result is
    // (2^255 - 1) / 10^20, which makes the largest exponent ln((2^255 - 1) / 10^20) = 130.700829182905140221.
    // The smallest possible result is 10^(-18), which makes largest negative argument
    // ln(10^(-18)) = -41.446531673892822312.
    // We use 130.0 and -41.0 to have some safety margin.
    int256 constant MAX_NATURAL_EXPONENT = 130e18;
    int256 constant MIN_NATURAL_EXPONENT = -41e18;

    // Bounds for ln_36's argument. Both ln(0.9) and ln(1.1) can be represented with 36 decimal places in a fixed point
    // 256 bit integer.
    int256 constant LN_36_LOWER_BOUND = ONE_18 - 1e17;
    int256 constant LN_36_UPPER_BOUND = ONE_18 + 1e17;

    uint256 constant MILD_EXPONENT_BOUND = 2 ** 254 / uint256(ONE_20);

    // 18 decimal constants
    int256 constant x0 = 128000000000000000000; // 2ˆ7
    int256 constant a0 = 38877084059945950922200000000000000000000000000000000000; // eˆ(x0) (no decimals)
    int256 constant x1 = 64000000000000000000; // 2ˆ6
    int256 constant a1 = 6235149080811616882910000000; // eˆ(x1) (no decimals)

    // 20 decimal constants
    int256 constant x2 = 3200000000000000000000; // 2ˆ5
    int256 constant a2 = 7896296018268069516100000000000000; // eˆ(x2)
    int256 constant x3 = 1600000000000000000000; // 2ˆ4
    int256 constant a3 = 888611052050787263676000000; // eˆ(x3)
    int256 constant x4 = 800000000000000000000; // 2ˆ3
    int256 constant a4 = 298095798704172827474000; // eˆ(x4)
    int256 constant x5 = 400000000000000000000; // 2ˆ2
    int256 constant a5 = 5459815003314423907810; // eˆ(x5)
    int256 constant x6 = 200000000000000000000; // 2ˆ1
    int256 constant a6 = 738905609893065022723; // eˆ(x6)
    int256 constant x7 = 100000000000000000000; // 2ˆ0
    int256 constant a7 = 271828182845904523536; // eˆ(x7)
    int256 constant x8 = 50000000000000000000; // 2ˆ-1
    int256 constant a8 = 164872127070012814685; // eˆ(x8)
    int256 constant x9 = 25000000000000000000; // 2ˆ-2
    int256 constant a9 = 128402541668774148407; // eˆ(x9)
    int256 constant x10 = 12500000000000000000; // 2ˆ-3
    int256 constant a10 = 113314845306682631683; // eˆ(x10)
    int256 constant x11 = 6250000000000000000; // 2ˆ-4
    int256 constant a11 = 106449445891785942956; // eˆ(x11)

    /**
     * @dev Exponentiation (x^y) with unsigned 18 decimal fixed point base and exponent.
     *
     * Reverts if ln(x) * y is smaller than `MIN_NATURAL_EXPONENT`, or larger than `MAX_NATURAL_EXPONENT`.
     */
    function pow(uint256 x, uint256 y) internal pure returns (uint256) {
        if (y == 0) {
            // We solve the 0^0 indetermination by making it equal one.
            return uint256(ONE_18);
        }

        if (x == 0) {
            return 0;
        }

        // Instead of computing x^y directly, we instead rely on the properties of logarithms and exponentiation to
        // arrive at that result. In particular, exp(ln(x)) = x, and ln(x^y) = y * ln(x). This means
        // x^y = exp(y * ln(x)).

        // The ln function takes a signed value, so we need to make sure x fits in the signed 256 bit range.
        if (x >> 255 != 0) {
            revert BaseOutOfBounds();
        }
        int256 x_int256 = int256(x);

        // We will compute y * ln(x) in a single step. Depending on the value of x, we can either use ln or ln_36. In
        // both cases, we leave the division by ONE_18 (due to fixed point multiplication) to the end.

        // This prevents y * ln(x) from overflowing, and at the same time guarantees y fits in the signed 256 bit range.
        if (y >= MILD_EXPONENT_BOUND) {
            revert ExponentOutOfBounds();
        }
        int256 y_int256 = int256(y);

        int256 logxTimes_y;
        unchecked {
            if (LN_36_LOWER_BOUND < x_int256 && x_int256 < LN_36_UPPER_BOUND) {
                int256 ln_36_x = _ln_36(x_int256);

                // ln_36_x has 36 decimal places, so multiplying by y_int256 isn't as straightforward, since we can't just
                // bring y_int256 to 36 decimal places, as it might overflow. Instead, we perform two 18 decimal
                // multiplications and add the results: one with the first 18 decimals of ln_36_x, and one with the
                // (downscaled) last 18 decimals.
                logxTimes_y = ((ln_36_x / ONE_18) * y_int256 + ((ln_36_x % ONE_18) * y_int256) / ONE_18);
            } else {
                logxTimes_y = _ln(x_int256) * y_int256;
            }
            logxTimes_y /= ONE_18;
        }

        // Finally, we compute exp(y * ln(x)) to arrive at x^y
        if (!(MIN_NATURAL_EXPONENT <= logxTimes_y && logxTimes_y <= MAX_NATURAL_EXPONENT)) {
            revert ProductOutOfBounds();
        }

        return uint256(exp(logxTimes_y));
    }

    /**
     * @dev Natural exponentiation (e^x) with signed 18 decimal fixed point exponent.
     *
     * Reverts if `x` is smaller than MIN_NATURAL_EXPONENT, or larger than `MAX_NATURAL_EXPONENT`.
     */
    function exp(int256 x) internal pure returns (int256) {
        if (!(x >= MIN_NATURAL_EXPONENT && x <= MAX_NATURAL_EXPONENT)) {
            revert InvalidExponent();
        }

        // We avoid using recursion here because zkSync doesn't support it.
        bool negativeExponent = false;

        if (x < 0) {
            // We only handle positive exponents: e^(-x) is computed as 1 / e^x. We can safely make x positive since it
            // fits in the signed 256 bit range (as it is larger than MIN_NATURAL_EXPONENT). In the negative
            // exponent case, compute e^x, then return 1 / result.
            unchecked {
                x = -x;
            }
            negativeExponent = true;
        }

        // First, we use the fact that e^(x+y) = e^x * e^y to decompose x into a sum of powers of two, which we call x_n,
        // where x_n == 2^(7 - n), and e^x_n = a_n has been precomputed. We choose the first x_n, x0, to equal 2^7
        // because all larger powers are larger than MAX_NATURAL_EXPONENT, and therefore not present in the
        // decomposition.
        // At the end of this process we will have the product of all e^x_n = a_n that apply, and the remainder of this
        // decomposition, which will be lower than the smallest x_n.
        // exp(x) = k_0 * a_0 * k_1 * a_1 * ... + k_n * a_n * exp(remainder), where each k_n equals either 0 or 1.
        // We mutate x by subtracting x_n, making it the remainder of the decomposition.

        // The first two a_n (e^(2^7) and e^(2^6)) are too large if stored as 18 decimal numbers, and could cause
        // intermediate overflows. Instead we store them as plain integers, with 0 decimals.
        // Additionally, x0 + x1 is larger than MAX_NATURAL_EXPONENT, which means they will not both be present in the
        // decomposition.

        // For each x_n, we test if that term is present in the decomposition (if x is larger than it), and if so deduct
        // it and compute the accumulated product.

        int256 firstAN;
        unchecked {
            if (x >= x0) {
                x -= x0;
                firstAN = a0;
            } else if (x >= x1) {
                x -= x1;
                firstAN = a1;
            } else {
                firstAN = 1; // One with no decimal places
            }

            // We now transform x into a 20 decimal fixed point number, to have enhanced precision when computing the
            // smaller terms.
            x *= 100;
        }

        // `product` is the accumulated product of all a_n (except a0 and a1), which starts at 20 decimal fixed point
        // one. Recall that fixed point multiplication requires dividing by ONE_20.
        int256 product = ONE_20;

        unchecked {
            if (x >= x2) {
                x -= x2;
                product = (product * a2) / ONE_20;
            }
            if (x >= x3) {
                x -= x3;
                product = (product * a3) / ONE_20;
            }
            if (x >= x4) {
                x -= x4;
                product = (product * a4) / ONE_20;
            }
            if (x >= x5) {
                x -= x5;
                product = (product * a5) / ONE_20;
            }
            if (x >= x6) {
                x -= x6;
                product = (product * a6) / ONE_20;
            }
            if (x >= x7) {
                x -= x7;
                product = (product * a7) / ONE_20;
            }
            if (x >= x8) {
                x -= x8;
                product = (product * a8) / ONE_20;
            }
            if (x >= x9) {
                x -= x9;
                product = (product * a9) / ONE_20;
            }
        }

        // x10 and x11 are unnecessary here since we have high enough precision already.

        // Now we need to compute e^x, where x is small (in particular, it is smaller than x9). We use the Taylor series
        // expansion for e^x: 1 + x + (x^2 / 2!) + (x^3 / 3!) + ... + (x^n / n!).

        int256 seriesSum = ONE_20; // The initial one in the sum, with 20 decimal places.
        int256 term; // Each term in the sum, where the nth term is (x^n / n!).

        // The first term is simply x.
        term = x;
        unchecked {
            seriesSum += term;

            // Each term (x^n / n!) equals the previous one times x, divided by n. Since x is a fixed point number,
            // multiplying by it requires dividing by ONE_20, but dividing by the non-fixed point n values does not.

            term = ((term * x) / ONE_20) / 2;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 3;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 4;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 5;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 6;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 7;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 8;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 9;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 10;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 11;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 12;
            seriesSum += term;

            // 12 Taylor terms are sufficient for 18 decimal precision.

            // We now have the first a_n (with no decimals), and the product of all other a_n present, and the Taylor
            // approximation of the exponentiation of the remainder (both with 20 decimals). All that remains is to multiply
            // all three (one 20 decimal fixed point multiplication, dividing by ONE_20, and one integer multiplication),
            // and then drop two digits to return an 18 decimal value.

            int256 result = (((product * seriesSum) / ONE_20) * firstAN) / 100;

            // We avoid using recursion here because zkSync doesn't support it.
            return negativeExponent ? (ONE_18 * ONE_18) / result : result;
        }
    }

    /// @dev Logarithm (log(arg, base), with signed 18 decimal fixed point base and argument.
    function log(int256 arg, int256 base) internal pure returns (int256) {
        // This performs a simple base change: log(arg, base) = ln(arg) / ln(base).

        // Both logBase and logArg are computed as 36 decimal fixed point numbers, either by using ln_36, or by
        // upscaling.

        int256 logBase;
        unchecked {
            if (LN_36_LOWER_BOUND < base && base < LN_36_UPPER_BOUND) {
                logBase = _ln_36(base);
            } else {
                logBase = _ln(base) * ONE_18;
            }
        }

        int256 logArg;
        unchecked {
            if (LN_36_LOWER_BOUND < arg && arg < LN_36_UPPER_BOUND) {
                logArg = _ln_36(arg);
            } else {
                logArg = _ln(arg) * ONE_18;
            }

            // When dividing, we multiply by ONE_18 to arrive at a result with 18 decimal places
            return (logArg * ONE_18) / logBase;
        }
    }

    /// @dev Natural logarithm (ln(a)) with signed 18 decimal fixed point argument.
    function ln(int256 a) internal pure returns (int256) {
        // The real natural logarithm is not defined for negative numbers or zero.
        if (a <= 0) {
            revert OutOfBounds();
        }
        if (LN_36_LOWER_BOUND < a && a < LN_36_UPPER_BOUND) {
            unchecked {
                return _ln_36(a) / ONE_18;
            }
        } else {
            return _ln(a);
        }
    }

    /// @dev Internal natural logarithm (ln(a)) with signed 18 decimal fixed point argument.
    function _ln(int256 a) private pure returns (int256) {
        // We avoid using recursion here because zkSync doesn't support it.
        bool negativeExponent = false;

        if (a < ONE_18) {
            // Since ln(a^k) = k * ln(a), we can compute ln(a) as ln(a) = ln((1/a)^(-1)) = - ln((1/a)). If a is less
            // than one, 1/a will be greater than one, so in this case we compute ln(1/a) and negate the final result.
            unchecked {
                a = (ONE_18 * ONE_18) / a;
            }
            negativeExponent = true;
        }

        // First, we use the fact that ln^(a * b) = ln(a) + ln(b) to decompose ln(a) into a sum of powers of two, which
        // we call x_n, where x_n == 2^(7 - n), which are the natural logarithm of precomputed quantities a_n (that is,
        // ln(a_n) = x_n). We choose the first x_n, x0, to equal 2^7 because the exponential of all larger powers cannot
        // be represented as 18 fixed point decimal numbers in 256 bits, and are therefore larger than a.
        // At the end of this process we will have the sum of all x_n = ln(a_n) that apply, and the remainder of this
        // decomposition, which will be lower than the smallest a_n.
        // ln(a) = k_0 * x_0 + k_1 * x_1 + ... + k_n * x_n + ln(remainder), where each k_n equals either 0 or 1.
        // We mutate a by subtracting a_n, making it the remainder of the decomposition.

        // For reasons related to how `exp` works, the first two a_n (e^(2^7) and e^(2^6)) are not stored as fixed point
        // numbers with 18 decimals, but instead as plain integers with 0 decimals, so we need to multiply them by
        // ONE_18 to convert them to fixed point.
        // For each a_n, we test if that term is present in the decomposition (if a is larger than it), and if so divide
        // by it and compute the accumulated sum.

        int256 sum = 0;
        unchecked {
            if (a >= a0 * ONE_18) {
                a /= a0; // Integer, not fixed point division
                sum += x0;
            }

            if (a >= a1 * ONE_18) {
                a /= a1; // Integer, not fixed point division
                sum += x1;
            }

            // All other a_n and x_n are stored as 20 digit fixed point numbers, so we convert the sum and a to this format.
            sum *= 100;
            a *= 100;

            // Because further a_n are  20 digit fixed point numbers, we multiply by ONE_20 when dividing by them.

            if (a >= a2) {
                a = (a * ONE_20) / a2;
                sum += x2;
            }

            if (a >= a3) {
                a = (a * ONE_20) / a3;
                sum += x3;
            }

            if (a >= a4) {
                a = (a * ONE_20) / a4;
                sum += x4;
            }

            if (a >= a5) {
                a = (a * ONE_20) / a5;
                sum += x5;
            }

            if (a >= a6) {
                a = (a * ONE_20) / a6;
                sum += x6;
            }

            if (a >= a7) {
                a = (a * ONE_20) / a7;
                sum += x7;
            }

            if (a >= a8) {
                a = (a * ONE_20) / a8;
                sum += x8;
            }

            if (a >= a9) {
                a = (a * ONE_20) / a9;
                sum += x9;
            }

            if (a >= a10) {
                a = (a * ONE_20) / a10;
                sum += x10;
            }

            if (a >= a11) {
                a = (a * ONE_20) / a11;
                sum += x11;
            }
        }

        // a is now a small number (smaller than a_11, which roughly equals 1.06). This means we can use a Taylor series
        // that converges rapidly for values of `a` close to one - the same one used in ln_36.
        // Let z = (a - 1) / (a + 1).
        // ln(a) = 2 * (z + z^3 / 3 + z^5 / 5 + z^7 / 7 + ... + z^(2 * n + 1) / (2 * n + 1))

        // Recall that 20 digit fixed point division requires multiplying by ONE_20, and multiplication requires
        // division by ONE_20.
        unchecked {
            int256 z = ((a - ONE_20) * ONE_20) / (a + ONE_20);
            int256 z_squared = (z * z) / ONE_20;

            // num is the numerator of the series: the z^(2 * n + 1) term
            int256 num = z;

            // seriesSum holds the accumulated sum of each term in the series, starting with the initial z
            int256 seriesSum = num;

            // In each step, the numerator is multiplied by z^2
            num = (num * z_squared) / ONE_20;
            seriesSum += num / 3;

            num = (num * z_squared) / ONE_20;
            seriesSum += num / 5;

            num = (num * z_squared) / ONE_20;
            seriesSum += num / 7;

            num = (num * z_squared) / ONE_20;
            seriesSum += num / 9;

            num = (num * z_squared) / ONE_20;
            seriesSum += num / 11;

            // 6 Taylor terms are sufficient for 36 decimal precision.

            // Finally, we multiply by 2 (non fixed point) to compute ln(remainder)
            seriesSum *= 2;

            // We now have the sum of all x_n present, and the Taylor approximation of the logarithm of the remainder (both
            // with 20 decimals). All that remains is to sum these two, and then drop two digits to return a 18 decimal
            // value.

            int256 result = (sum + seriesSum) / 100;

            // We avoid using recursion here because zkSync doesn't support it.
            return negativeExponent ? -result : result;
        }
    }

    /**
     * @dev Internal high precision (36 decimal places) natural logarithm (ln(x)) with signed 18 decimal fixed point argument,
     * for x close to one.
     *
     * Should only be used if x is between LN_36_LOWER_BOUND and LN_36_UPPER_BOUND.
     */
    function _ln_36(int256 x) private pure returns (int256) {
        // Since ln(1) = 0, a value of x close to one will yield a very small result, which makes using 36 digits
        // worthwhile.

        // First, we transform x to a 36 digit fixed point value.
        unchecked {
            x *= ONE_18;

            // We will use the following Taylor expansion, which converges very rapidly. Let z = (x - 1) / (x + 1).
            // ln(x) = 2 * (z + z^3 / 3 + z^5 / 5 + z^7 / 7 + ... + z^(2 * n + 1) / (2 * n + 1))

            // Recall that 36 digit fixed point division requires multiplying by ONE_36, and multiplication requires
            // division by ONE_36.
            int256 z = ((x - ONE_36) * ONE_36) / (x + ONE_36);
            int256 z_squared = (z * z) / ONE_36;

            // num is the numerator of the series: the z^(2 * n + 1) term
            int256 num = z;

            // seriesSum holds the accumulated sum of each term in the series, starting with the initial z
            int256 seriesSum = num;

            // In each step, the numerator is multiplied by z^2
            num = (num * z_squared) / ONE_36;
            seriesSum += num / 3;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 5;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 7;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 9;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 11;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 13;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 15;

            // 8 Taylor terms are sufficient for 36 decimal precision.

            // All that remains is multiplying by 2 (non fixed point).
            return seriesSum * 2;
        }
    }
}

// pkg/solidity-utils/contracts/math/FixedPoint.sol

/// @notice Support 18-decimal fixed point arithmetic. All Vault calculations use this for high and uniform precision.
library FixedPoint {
    /// @notice Attempted division by zero.
    error ZeroDivision();

    // solhint-disable no-inline-assembly
    // solhint-disable private-vars-leading-underscore

    uint256 internal constant ONE = 1e18; // 18 decimal places
    uint256 internal constant TWO = 2 * ONE;
    uint256 internal constant FOUR = 4 * ONE;
    uint256 internal constant MAX_POW_RELATIVE_ERROR = 10000; // 10^(-14)

    function mulDown(uint256 a, uint256 b) internal pure returns (uint256) {
        // Multiplication overflow protection is provided by Solidity 0.8.x.
        uint256 product = a * b;

        return product / ONE;
    }

    function mulUp(uint256 a, uint256 b) internal pure returns (uint256 result) {
        // Multiplication overflow protection is provided by Solidity 0.8.x.
        uint256 product = a * b;

        // Equivalent to:
        // result = product == 0 ? 0 : ((product - 1) / FixedPoint.ONE) + 1
        assembly ("memory-safe") {
            result := mul(iszero(iszero(product)), add(div(sub(product, 1), ONE), 1))
        }
    }

    function divDown(uint256 a, uint256 b) internal pure returns (uint256) {
        // Solidity 0.8 reverts with a Panic code (0x11) if the multiplication overflows.
        uint256 aInflated = a * ONE;

        // Solidity 0.8 reverts with a "Division by Zero" Panic code (0x12) if b is zero
        return aInflated / b;
    }

    function divUp(uint256 a, uint256 b) internal pure returns (uint256 result) {
        return mulDivUp(a, ONE, b);
    }

    /// @dev Return (a * b) / c, rounding up.
    function mulDivUp(uint256 a, uint256 b, uint256 c) internal pure returns (uint256 result) {
        // This check is required because Yul's `div` doesn't revert on c==0.
        if (c == 0) {
            revert ZeroDivision();
        }

        // Multiple overflow protection is done by Solidity 0.8.x.
        uint256 product = a * b;

        // The traditional divUp formula is:
        // divUp(x, y) := (x + y - 1) / y
        // To avoid intermediate overflow in the addition, we distribute the division and get:
        // divUp(x, y) := (x - 1) / y + 1
        // Note that this requires x != 0, if x == 0 then the result is zero
        //
        // Equivalent to:
        // result = a == 0 ? 0 : (a * b - 1) / c + 1
        assembly ("memory-safe") {
            result := mul(iszero(iszero(product)), add(div(sub(product, 1), c), 1))
        }
    }

    /**
     * @dev Version of divUp when the input is raw (i.e., already "inflated"). For instance,
     * invariant * invariant (36 decimals) vs. invariant.mulDown(invariant) (18 decimal FP).
     * This can occur in calculations with many successive multiplications and divisions, and
     * we want to minimize the number of operations by avoiding unnecessary scaling by ONE.
     */
    function divUpRaw(uint256 a, uint256 b) internal pure returns (uint256 result) {
        // This check is required because Yul's `div` doesn't revert on b==0.
        if (b == 0) {
            revert ZeroDivision();
        }

        // Equivalent to:
        // result = a == 0 ? 0 : 1 + (a - 1) / b
        assembly ("memory-safe") {
            result := mul(iszero(iszero(a)), add(1, div(sub(a, 1), b)))
        }
    }

    /**
     * @dev Returns x^y, assuming both are fixed point numbers, rounding down. The result is guaranteed to not be above
     * the true value (that is, the error function expected - actual is always positive).
     */
    function powDown(uint256 x, uint256 y) internal pure returns (uint256) {
        // Optimize for when y equals 1.0, 2.0 or 4.0, as those are very simple to implement and occur often in 50/50
        // and 80/20 Weighted Pools
        if (y == ONE) {
            return x;
        } else if (y == TWO) {
            return mulDown(x, x);
        } else if (y == FOUR) {
            uint256 square = mulDown(x, x);
            return mulDown(square, square);
        } else {
            uint256 raw = LogExpMath.pow(x, y);
            uint256 maxError = mulUp(raw, MAX_POW_RELATIVE_ERROR) + 1;

            if (raw < maxError) {
                return 0;
            } else {
                unchecked {
                    return raw - maxError;
                }
            }
        }
    }

    /**
     * @dev Returns x^y, assuming both are fixed point numbers, rounding up. The result is guaranteed to not be below
     * the true value (that is, the error function expected - actual is always negative).
     */
    function powUp(uint256 x, uint256 y) internal pure returns (uint256) {
        // Optimize for when y equals 1.0, 2.0 or 4.0, as those are very simple to implement and occur often in 50/50
        // and 80/20 Weighted Pools
        if (y == ONE) {
            return x;
        } else if (y == TWO) {
            return mulUp(x, x);
        } else if (y == FOUR) {
            uint256 square = mulUp(x, x);
            return mulUp(square, square);
        } else {
            uint256 raw = LogExpMath.pow(x, y);
            uint256 maxError = mulUp(raw, MAX_POW_RELATIVE_ERROR) + 1;

            return raw + maxError;
        }
    }

    /**
     * @dev Returns the complement of a value (1 - x), capped to 0 if x is larger than 1.
     *
     * Useful when computing the complement for values with some level of relative error, as it strips this error and
     * prevents intermediate negative values.
     */
    function complement(uint256 x) internal pure returns (uint256 result) {
        // Equivalent to:
        // result = (x < ONE) ? (ONE - x) : 0
        assembly ("memory-safe") {
            result := mul(lt(x, ONE), sub(ONE, x))
        }
    }
}

```


## ./pkg/solidity-utils/contracts/math/LogExpMath.sol

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

// pkg/solidity-utils/contracts/math/LogExpMath.sol

// solhint-disable

/**
 * @dev Exponentiation and logarithm functions for 18 decimal fixed point numbers (both base and exponent/argument).
 *
 * Exponentiation and logarithm with arbitrary bases (x^y and log_x(y)) are implemented by conversion to natural
 * exponentiation and logarithm (where the base is Euler's number).
 *
 * All math operations are unchecked in order to save gas.
 *
 * @author Fernando Martinelli - @fernandomartinelli
 * @author Sergio Yuhjtman     - @sergioyuhjtman
 * @author Daniel Fernandez    - @dmf7z
 */
library LogExpMath {
    /// @notice This error is thrown when a base is not within an acceptable range.
    error BaseOutOfBounds();

    /// @notice This error is thrown when a exponent is not within an acceptable range.
    error ExponentOutOfBounds();

    /// @notice This error is thrown when the exponent * ln(base) is not within an acceptable range.
    error ProductOutOfBounds();

    /// @notice This error is thrown when an exponent used in the exp function is not within an acceptable range.
    error InvalidExponent();

    /// @notice This error is thrown when a variable or result is not within the acceptable bounds defined in the function.
    error OutOfBounds();

    // All fixed point multiplications and divisions are inlined. This means we need to divide by ONE when multiplying
    // two numbers, and multiply by ONE when dividing them.

    // All arguments and return values are 18 decimal fixed point numbers.
    int256 constant ONE_18 = 1e18;

    // Internally, intermediate values are computed with higher precision as 20 decimal fixed point numbers, and in the
    // case of ln36, 36 decimals.
    int256 constant ONE_20 = 1e20;
    int256 constant ONE_36 = 1e36;

    // The domain of natural exponentiation is bound by the word size and number of decimals used.
    //
    // Because internally the result will be stored using 20 decimals, the largest possible result is
    // (2^255 - 1) / 10^20, which makes the largest exponent ln((2^255 - 1) / 10^20) = 130.700829182905140221.
    // The smallest possible result is 10^(-18), which makes largest negative argument
    // ln(10^(-18)) = -41.446531673892822312.
    // We use 130.0 and -41.0 to have some safety margin.
    int256 constant MAX_NATURAL_EXPONENT = 130e18;
    int256 constant MIN_NATURAL_EXPONENT = -41e18;

    // Bounds for ln_36's argument. Both ln(0.9) and ln(1.1) can be represented with 36 decimal places in a fixed point
    // 256 bit integer.
    int256 constant LN_36_LOWER_BOUND = ONE_18 - 1e17;
    int256 constant LN_36_UPPER_BOUND = ONE_18 + 1e17;

    uint256 constant MILD_EXPONENT_BOUND = 2 ** 254 / uint256(ONE_20);

    // 18 decimal constants
    int256 constant x0 = 128000000000000000000; // 2ˆ7
    int256 constant a0 = 38877084059945950922200000000000000000000000000000000000; // eˆ(x0) (no decimals)
    int256 constant x1 = 64000000000000000000; // 2ˆ6
    int256 constant a1 = 6235149080811616882910000000; // eˆ(x1) (no decimals)

    // 20 decimal constants
    int256 constant x2 = 3200000000000000000000; // 2ˆ5
    int256 constant a2 = 7896296018268069516100000000000000; // eˆ(x2)
    int256 constant x3 = 1600000000000000000000; // 2ˆ4
    int256 constant a3 = 888611052050787263676000000; // eˆ(x3)
    int256 constant x4 = 800000000000000000000; // 2ˆ3
    int256 constant a4 = 298095798704172827474000; // eˆ(x4)
    int256 constant x5 = 400000000000000000000; // 2ˆ2
    int256 constant a5 = 5459815003314423907810; // eˆ(x5)
    int256 constant x6 = 200000000000000000000; // 2ˆ1
    int256 constant a6 = 738905609893065022723; // eˆ(x6)
    int256 constant x7 = 100000000000000000000; // 2ˆ0
    int256 constant a7 = 271828182845904523536; // eˆ(x7)
    int256 constant x8 = 50000000000000000000; // 2ˆ-1
    int256 constant a8 = 164872127070012814685; // eˆ(x8)
    int256 constant x9 = 25000000000000000000; // 2ˆ-2
    int256 constant a9 = 128402541668774148407; // eˆ(x9)
    int256 constant x10 = 12500000000000000000; // 2ˆ-3
    int256 constant a10 = 113314845306682631683; // eˆ(x10)
    int256 constant x11 = 6250000000000000000; // 2ˆ-4
    int256 constant a11 = 106449445891785942956; // eˆ(x11)

    /**
     * @dev Exponentiation (x^y) with unsigned 18 decimal fixed point base and exponent.
     *
     * Reverts if ln(x) * y is smaller than `MIN_NATURAL_EXPONENT`, or larger than `MAX_NATURAL_EXPONENT`.
     */
    function pow(uint256 x, uint256 y) internal pure returns (uint256) {
        if (y == 0) {
            // We solve the 0^0 indetermination by making it equal one.
            return uint256(ONE_18);
        }

        if (x == 0) {
            return 0;
        }

        // Instead of computing x^y directly, we instead rely on the properties of logarithms and exponentiation to
        // arrive at that result. In particular, exp(ln(x)) = x, and ln(x^y) = y * ln(x). This means
        // x^y = exp(y * ln(x)).

        // The ln function takes a signed value, so we need to make sure x fits in the signed 256 bit range.
        if (x >> 255 != 0) {
            revert BaseOutOfBounds();
        }
        int256 x_int256 = int256(x);

        // We will compute y * ln(x) in a single step. Depending on the value of x, we can either use ln or ln_36. In
        // both cases, we leave the division by ONE_18 (due to fixed point multiplication) to the end.

        // This prevents y * ln(x) from overflowing, and at the same time guarantees y fits in the signed 256 bit range.
        if (y >= MILD_EXPONENT_BOUND) {
            revert ExponentOutOfBounds();
        }
        int256 y_int256 = int256(y);

        int256 logxTimes_y;
        unchecked {
            if (LN_36_LOWER_BOUND < x_int256 && x_int256 < LN_36_UPPER_BOUND) {
                int256 ln_36_x = _ln_36(x_int256);

                // ln_36_x has 36 decimal places, so multiplying by y_int256 isn't as straightforward, since we can't just
                // bring y_int256 to 36 decimal places, as it might overflow. Instead, we perform two 18 decimal
                // multiplications and add the results: one with the first 18 decimals of ln_36_x, and one with the
                // (downscaled) last 18 decimals.
                logxTimes_y = ((ln_36_x / ONE_18) * y_int256 + ((ln_36_x % ONE_18) * y_int256) / ONE_18);
            } else {
                logxTimes_y = _ln(x_int256) * y_int256;
            }
            logxTimes_y /= ONE_18;
        }

        // Finally, we compute exp(y * ln(x)) to arrive at x^y
        if (!(MIN_NATURAL_EXPONENT <= logxTimes_y && logxTimes_y <= MAX_NATURAL_EXPONENT)) {
            revert ProductOutOfBounds();
        }

        return uint256(exp(logxTimes_y));
    }

    /**
     * @dev Natural exponentiation (e^x) with signed 18 decimal fixed point exponent.
     *
     * Reverts if `x` is smaller than MIN_NATURAL_EXPONENT, or larger than `MAX_NATURAL_EXPONENT`.
     */
    function exp(int256 x) internal pure returns (int256) {
        if (!(x >= MIN_NATURAL_EXPONENT && x <= MAX_NATURAL_EXPONENT)) {
            revert InvalidExponent();
        }

        // We avoid using recursion here because zkSync doesn't support it.
        bool negativeExponent = false;

        if (x < 0) {
            // We only handle positive exponents: e^(-x) is computed as 1 / e^x. We can safely make x positive since it
            // fits in the signed 256 bit range (as it is larger than MIN_NATURAL_EXPONENT). In the negative
            // exponent case, compute e^x, then return 1 / result.
            unchecked {
                x = -x;
            }
            negativeExponent = true;
        }

        // First, we use the fact that e^(x+y) = e^x * e^y to decompose x into a sum of powers of two, which we call x_n,
        // where x_n == 2^(7 - n), and e^x_n = a_n has been precomputed. We choose the first x_n, x0, to equal 2^7
        // because all larger powers are larger than MAX_NATURAL_EXPONENT, and therefore not present in the
        // decomposition.
        // At the end of this process we will have the product of all e^x_n = a_n that apply, and the remainder of this
        // decomposition, which will be lower than the smallest x_n.
        // exp(x) = k_0 * a_0 * k_1 * a_1 * ... + k_n * a_n * exp(remainder), where each k_n equals either 0 or 1.
        // We mutate x by subtracting x_n, making it the remainder of the decomposition.

        // The first two a_n (e^(2^7) and e^(2^6)) are too large if stored as 18 decimal numbers, and could cause
        // intermediate overflows. Instead we store them as plain integers, with 0 decimals.
        // Additionally, x0 + x1 is larger than MAX_NATURAL_EXPONENT, which means they will not both be present in the
        // decomposition.

        // For each x_n, we test if that term is present in the decomposition (if x is larger than it), and if so deduct
        // it and compute the accumulated product.

        int256 firstAN;
        unchecked {
            if (x >= x0) {
                x -= x0;
                firstAN = a0;
            } else if (x >= x1) {
                x -= x1;
                firstAN = a1;
            } else {
                firstAN = 1; // One with no decimal places
            }

            // We now transform x into a 20 decimal fixed point number, to have enhanced precision when computing the
            // smaller terms.
            x *= 100;
        }

        // `product` is the accumulated product of all a_n (except a0 and a1), which starts at 20 decimal fixed point
        // one. Recall that fixed point multiplication requires dividing by ONE_20.
        int256 product = ONE_20;

        unchecked {
            if (x >= x2) {
                x -= x2;
                product = (product * a2) / ONE_20;
            }
            if (x >= x3) {
                x -= x3;
                product = (product * a3) / ONE_20;
            }
            if (x >= x4) {
                x -= x4;
                product = (product * a4) / ONE_20;
            }
            if (x >= x5) {
                x -= x5;
                product = (product * a5) / ONE_20;
            }
            if (x >= x6) {
                x -= x6;
                product = (product * a6) / ONE_20;
            }
            if (x >= x7) {
                x -= x7;
                product = (product * a7) / ONE_20;
            }
            if (x >= x8) {
                x -= x8;
                product = (product * a8) / ONE_20;
            }
            if (x >= x9) {
                x -= x9;
                product = (product * a9) / ONE_20;
            }
        }

        // x10 and x11 are unnecessary here since we have high enough precision already.

        // Now we need to compute e^x, where x is small (in particular, it is smaller than x9). We use the Taylor series
        // expansion for e^x: 1 + x + (x^2 / 2!) + (x^3 / 3!) + ... + (x^n / n!).

        int256 seriesSum = ONE_20; // The initial one in the sum, with 20 decimal places.
        int256 term; // Each term in the sum, where the nth term is (x^n / n!).

        // The first term is simply x.
        term = x;
        unchecked {
            seriesSum += term;

            // Each term (x^n / n!) equals the previous one times x, divided by n. Since x is a fixed point number,
            // multiplying by it requires dividing by ONE_20, but dividing by the non-fixed point n values does not.

            term = ((term * x) / ONE_20) / 2;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 3;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 4;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 5;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 6;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 7;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 8;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 9;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 10;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 11;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 12;
            seriesSum += term;

            // 12 Taylor terms are sufficient for 18 decimal precision.

            // We now have the first a_n (with no decimals), and the product of all other a_n present, and the Taylor
            // approximation of the exponentiation of the remainder (both with 20 decimals). All that remains is to multiply
            // all three (one 20 decimal fixed point multiplication, dividing by ONE_20, and one integer multiplication),
            // and then drop two digits to return an 18 decimal value.

            int256 result = (((product * seriesSum) / ONE_20) * firstAN) / 100;

            // We avoid using recursion here because zkSync doesn't support it.
            return negativeExponent ? (ONE_18 * ONE_18) / result : result;
        }
    }

    /// @dev Logarithm (log(arg, base), with signed 18 decimal fixed point base and argument.
    function log(int256 arg, int256 base) internal pure returns (int256) {
        // This performs a simple base change: log(arg, base) = ln(arg) / ln(base).

        // Both logBase and logArg are computed as 36 decimal fixed point numbers, either by using ln_36, or by
        // upscaling.

        int256 logBase;
        unchecked {
            if (LN_36_LOWER_BOUND < base && base < LN_36_UPPER_BOUND) {
                logBase = _ln_36(base);
            } else {
                logBase = _ln(base) * ONE_18;
            }
        }

        int256 logArg;
        unchecked {
            if (LN_36_LOWER_BOUND < arg && arg < LN_36_UPPER_BOUND) {
                logArg = _ln_36(arg);
            } else {
                logArg = _ln(arg) * ONE_18;
            }

            // When dividing, we multiply by ONE_18 to arrive at a result with 18 decimal places
            return (logArg * ONE_18) / logBase;
        }
    }

    /// @dev Natural logarithm (ln(a)) with signed 18 decimal fixed point argument.
    function ln(int256 a) internal pure returns (int256) {
        // The real natural logarithm is not defined for negative numbers or zero.
        if (a <= 0) {
            revert OutOfBounds();
        }
        if (LN_36_LOWER_BOUND < a && a < LN_36_UPPER_BOUND) {
            unchecked {
                return _ln_36(a) / ONE_18;
            }
        } else {
            return _ln(a);
        }
    }

    /// @dev Internal natural logarithm (ln(a)) with signed 18 decimal fixed point argument.
    function _ln(int256 a) private pure returns (int256) {
        // We avoid using recursion here because zkSync doesn't support it.
        bool negativeExponent = false;

        if (a < ONE_18) {
            // Since ln(a^k) = k * ln(a), we can compute ln(a) as ln(a) = ln((1/a)^(-1)) = - ln((1/a)). If a is less
            // than one, 1/a will be greater than one, so in this case we compute ln(1/a) and negate the final result.
            unchecked {
                a = (ONE_18 * ONE_18) / a;
            }
            negativeExponent = true;
        }

        // First, we use the fact that ln^(a * b) = ln(a) + ln(b) to decompose ln(a) into a sum of powers of two, which
        // we call x_n, where x_n == 2^(7 - n), which are the natural logarithm of precomputed quantities a_n (that is,
        // ln(a_n) = x_n). We choose the first x_n, x0, to equal 2^7 because the exponential of all larger powers cannot
        // be represented as 18 fixed point decimal numbers in 256 bits, and are therefore larger than a.
        // At the end of this process we will have the sum of all x_n = ln(a_n) that apply, and the remainder of this
        // decomposition, which will be lower than the smallest a_n.
        // ln(a) = k_0 * x_0 + k_1 * x_1 + ... + k_n * x_n + ln(remainder), where each k_n equals either 0 or 1.
        // We mutate a by subtracting a_n, making it the remainder of the decomposition.

        // For reasons related to how `exp` works, the first two a_n (e^(2^7) and e^(2^6)) are not stored as fixed point
        // numbers with 18 decimals, but instead as plain integers with 0 decimals, so we need to multiply them by
        // ONE_18 to convert them to fixed point.
        // For each a_n, we test if that term is present in the decomposition (if a is larger than it), and if so divide
        // by it and compute the accumulated sum.

        int256 sum = 0;
        unchecked {
            if (a >= a0 * ONE_18) {
                a /= a0; // Integer, not fixed point division
                sum += x0;
            }

            if (a >= a1 * ONE_18) {
                a /= a1; // Integer, not fixed point division
                sum += x1;
            }

            // All other a_n and x_n are stored as 20 digit fixed point numbers, so we convert the sum and a to this format.
            sum *= 100;
            a *= 100;

            // Because further a_n are  20 digit fixed point numbers, we multiply by ONE_20 when dividing by them.

            if (a >= a2) {
                a = (a * ONE_20) / a2;
                sum += x2;
            }

            if (a >= a3) {
                a = (a * ONE_20) / a3;
                sum += x3;
            }

            if (a >= a4) {
                a = (a * ONE_20) / a4;
                sum += x4;
            }

            if (a >= a5) {
                a = (a * ONE_20) / a5;
                sum += x5;
            }

            if (a >= a6) {
                a = (a * ONE_20) / a6;
                sum += x6;
            }

            if (a >= a7) {
                a = (a * ONE_20) / a7;
                sum += x7;
            }

            if (a >= a8) {
                a = (a * ONE_20) / a8;
                sum += x8;
            }

            if (a >= a9) {
                a = (a * ONE_20) / a9;
                sum += x9;
            }

            if (a >= a10) {
                a = (a * ONE_20) / a10;
                sum += x10;
            }

            if (a >= a11) {
                a = (a * ONE_20) / a11;
                sum += x11;
            }
        }

        // a is now a small number (smaller than a_11, which roughly equals 1.06). This means we can use a Taylor series
        // that converges rapidly for values of `a` close to one - the same one used in ln_36.
        // Let z = (a - 1) / (a + 1).
        // ln(a) = 2 * (z + z^3 / 3 + z^5 / 5 + z^7 / 7 + ... + z^(2 * n + 1) / (2 * n + 1))

        // Recall that 20 digit fixed point division requires multiplying by ONE_20, and multiplication requires
        // division by ONE_20.
        unchecked {
            int256 z = ((a - ONE_20) * ONE_20) / (a + ONE_20);
            int256 z_squared = (z * z) / ONE_20;

            // num is the numerator of the series: the z^(2 * n + 1) term
            int256 num = z;

            // seriesSum holds the accumulated sum of each term in the series, starting with the initial z
            int256 seriesSum = num;

            // In each step, the numerator is multiplied by z^2
            num = (num * z_squared) / ONE_20;
            seriesSum += num / 3;

            num = (num * z_squared) / ONE_20;
            seriesSum += num / 5;

            num = (num * z_squared) / ONE_20;
            seriesSum += num / 7;

            num = (num * z_squared) / ONE_20;
            seriesSum += num / 9;

            num = (num * z_squared) / ONE_20;
            seriesSum += num / 11;

            // 6 Taylor terms are sufficient for 36 decimal precision.

            // Finally, we multiply by 2 (non fixed point) to compute ln(remainder)
            seriesSum *= 2;

            // We now have the sum of all x_n present, and the Taylor approximation of the logarithm of the remainder (both
            // with 20 decimals). All that remains is to sum these two, and then drop two digits to return a 18 decimal
            // value.

            int256 result = (sum + seriesSum) / 100;

            // We avoid using recursion here because zkSync doesn't support it.
            return negativeExponent ? -result : result;
        }
    }

    /**
     * @dev Internal high precision (36 decimal places) natural logarithm (ln(x)) with signed 18 decimal fixed point argument,
     * for x close to one.
     *
     * Should only be used if x is between LN_36_LOWER_BOUND and LN_36_UPPER_BOUND.
     */
    function _ln_36(int256 x) private pure returns (int256) {
        // Since ln(1) = 0, a value of x close to one will yield a very small result, which makes using 36 digits
        // worthwhile.

        // First, we transform x to a 36 digit fixed point value.
        unchecked {
            x *= ONE_18;

            // We will use the following Taylor expansion, which converges very rapidly. Let z = (x - 1) / (x + 1).
            // ln(x) = 2 * (z + z^3 / 3 + z^5 / 5 + z^7 / 7 + ... + z^(2 * n + 1) / (2 * n + 1))

            // Recall that 36 digit fixed point division requires multiplying by ONE_36, and multiplication requires
            // division by ONE_36.
            int256 z = ((x - ONE_36) * ONE_36) / (x + ONE_36);
            int256 z_squared = (z * z) / ONE_36;

            // num is the numerator of the series: the z^(2 * n + 1) term
            int256 num = z;

            // seriesSum holds the accumulated sum of each term in the series, starting with the initial z
            int256 seriesSum = num;

            // In each step, the numerator is multiplied by z^2
            num = (num * z_squared) / ONE_36;
            seriesSum += num / 3;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 5;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 7;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 9;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 11;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 13;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 15;

            // 8 Taylor terms are sufficient for 36 decimal precision.

            // All that remains is multiplying by 2 (non fixed point).
            return seriesSum * 2;
        }
    }
}

```


## ./pkg/solidity-utils/contracts/math/StableMath.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/solidity-utils/contracts/math/LogExpMath.sol

// solhint-disable

/**
 * @dev Exponentiation and logarithm functions for 18 decimal fixed point numbers (both base and exponent/argument).
 *
 * Exponentiation and logarithm with arbitrary bases (x^y and log_x(y)) are implemented by conversion to natural
 * exponentiation and logarithm (where the base is Euler's number).
 *
 * All math operations are unchecked in order to save gas.
 *
 * @author Fernando Martinelli - @fernandomartinelli
 * @author Sergio Yuhjtman     - @sergioyuhjtman
 * @author Daniel Fernandez    - @dmf7z
 */
library LogExpMath {
    /// @notice This error is thrown when a base is not within an acceptable range.
    error BaseOutOfBounds();

    /// @notice This error is thrown when a exponent is not within an acceptable range.
    error ExponentOutOfBounds();

    /// @notice This error is thrown when the exponent * ln(base) is not within an acceptable range.
    error ProductOutOfBounds();

    /// @notice This error is thrown when an exponent used in the exp function is not within an acceptable range.
    error InvalidExponent();

    /// @notice This error is thrown when a variable or result is not within the acceptable bounds defined in the function.
    error OutOfBounds();

    // All fixed point multiplications and divisions are inlined. This means we need to divide by ONE when multiplying
    // two numbers, and multiply by ONE when dividing them.

    // All arguments and return values are 18 decimal fixed point numbers.
    int256 constant ONE_18 = 1e18;

    // Internally, intermediate values are computed with higher precision as 20 decimal fixed point numbers, and in the
    // case of ln36, 36 decimals.
    int256 constant ONE_20 = 1e20;
    int256 constant ONE_36 = 1e36;

    // The domain of natural exponentiation is bound by the word size and number of decimals used.
    //
    // Because internally the result will be stored using 20 decimals, the largest possible result is
    // (2^255 - 1) / 10^20, which makes the largest exponent ln((2^255 - 1) / 10^20) = 130.700829182905140221.
    // The smallest possible result is 10^(-18), which makes largest negative argument
    // ln(10^(-18)) = -41.446531673892822312.
    // We use 130.0 and -41.0 to have some safety margin.
    int256 constant MAX_NATURAL_EXPONENT = 130e18;
    int256 constant MIN_NATURAL_EXPONENT = -41e18;

    // Bounds for ln_36's argument. Both ln(0.9) and ln(1.1) can be represented with 36 decimal places in a fixed point
    // 256 bit integer.
    int256 constant LN_36_LOWER_BOUND = ONE_18 - 1e17;
    int256 constant LN_36_UPPER_BOUND = ONE_18 + 1e17;

    uint256 constant MILD_EXPONENT_BOUND = 2 ** 254 / uint256(ONE_20);

    // 18 decimal constants
    int256 constant x0 = 128000000000000000000; // 2ˆ7
    int256 constant a0 = 38877084059945950922200000000000000000000000000000000000; // eˆ(x0) (no decimals)
    int256 constant x1 = 64000000000000000000; // 2ˆ6
    int256 constant a1 = 6235149080811616882910000000; // eˆ(x1) (no decimals)

    // 20 decimal constants
    int256 constant x2 = 3200000000000000000000; // 2ˆ5
    int256 constant a2 = 7896296018268069516100000000000000; // eˆ(x2)
    int256 constant x3 = 1600000000000000000000; // 2ˆ4
    int256 constant a3 = 888611052050787263676000000; // eˆ(x3)
    int256 constant x4 = 800000000000000000000; // 2ˆ3
    int256 constant a4 = 298095798704172827474000; // eˆ(x4)
    int256 constant x5 = 400000000000000000000; // 2ˆ2
    int256 constant a5 = 5459815003314423907810; // eˆ(x5)
    int256 constant x6 = 200000000000000000000; // 2ˆ1
    int256 constant a6 = 738905609893065022723; // eˆ(x6)
    int256 constant x7 = 100000000000000000000; // 2ˆ0
    int256 constant a7 = 271828182845904523536; // eˆ(x7)
    int256 constant x8 = 50000000000000000000; // 2ˆ-1
    int256 constant a8 = 164872127070012814685; // eˆ(x8)
    int256 constant x9 = 25000000000000000000; // 2ˆ-2
    int256 constant a9 = 128402541668774148407; // eˆ(x9)
    int256 constant x10 = 12500000000000000000; // 2ˆ-3
    int256 constant a10 = 113314845306682631683; // eˆ(x10)
    int256 constant x11 = 6250000000000000000; // 2ˆ-4
    int256 constant a11 = 106449445891785942956; // eˆ(x11)

    /**
     * @dev Exponentiation (x^y) with unsigned 18 decimal fixed point base and exponent.
     *
     * Reverts if ln(x) * y is smaller than `MIN_NATURAL_EXPONENT`, or larger than `MAX_NATURAL_EXPONENT`.
     */
    function pow(uint256 x, uint256 y) internal pure returns (uint256) {
        if (y == 0) {
            // We solve the 0^0 indetermination by making it equal one.
            return uint256(ONE_18);
        }

        if (x == 0) {
            return 0;
        }

        // Instead of computing x^y directly, we instead rely on the properties of logarithms and exponentiation to
        // arrive at that result. In particular, exp(ln(x)) = x, and ln(x^y) = y * ln(x). This means
        // x^y = exp(y * ln(x)).

        // The ln function takes a signed value, so we need to make sure x fits in the signed 256 bit range.
        if (x >> 255 != 0) {
            revert BaseOutOfBounds();
        }
        int256 x_int256 = int256(x);

        // We will compute y * ln(x) in a single step. Depending on the value of x, we can either use ln or ln_36. In
        // both cases, we leave the division by ONE_18 (due to fixed point multiplication) to the end.

        // This prevents y * ln(x) from overflowing, and at the same time guarantees y fits in the signed 256 bit range.
        if (y >= MILD_EXPONENT_BOUND) {
            revert ExponentOutOfBounds();
        }
        int256 y_int256 = int256(y);

        int256 logxTimes_y;
        unchecked {
            if (LN_36_LOWER_BOUND < x_int256 && x_int256 < LN_36_UPPER_BOUND) {
                int256 ln_36_x = _ln_36(x_int256);

                // ln_36_x has 36 decimal places, so multiplying by y_int256 isn't as straightforward, since we can't just
                // bring y_int256 to 36 decimal places, as it might overflow. Instead, we perform two 18 decimal
                // multiplications and add the results: one with the first 18 decimals of ln_36_x, and one with the
                // (downscaled) last 18 decimals.
                logxTimes_y = ((ln_36_x / ONE_18) * y_int256 + ((ln_36_x % ONE_18) * y_int256) / ONE_18);
            } else {
                logxTimes_y = _ln(x_int256) * y_int256;
            }
            logxTimes_y /= ONE_18;
        }

        // Finally, we compute exp(y * ln(x)) to arrive at x^y
        if (!(MIN_NATURAL_EXPONENT <= logxTimes_y && logxTimes_y <= MAX_NATURAL_EXPONENT)) {
            revert ProductOutOfBounds();
        }

        return uint256(exp(logxTimes_y));
    }

    /**
     * @dev Natural exponentiation (e^x) with signed 18 decimal fixed point exponent.
     *
     * Reverts if `x` is smaller than MIN_NATURAL_EXPONENT, or larger than `MAX_NATURAL_EXPONENT`.
     */
    function exp(int256 x) internal pure returns (int256) {
        if (!(x >= MIN_NATURAL_EXPONENT && x <= MAX_NATURAL_EXPONENT)) {
            revert InvalidExponent();
        }

        // We avoid using recursion here because zkSync doesn't support it.
        bool negativeExponent = false;

        if (x < 0) {
            // We only handle positive exponents: e^(-x) is computed as 1 / e^x. We can safely make x positive since it
            // fits in the signed 256 bit range (as it is larger than MIN_NATURAL_EXPONENT). In the negative
            // exponent case, compute e^x, then return 1 / result.
            unchecked {
                x = -x;
            }
            negativeExponent = true;
        }

        // First, we use the fact that e^(x+y) = e^x * e^y to decompose x into a sum of powers of two, which we call x_n,
        // where x_n == 2^(7 - n), and e^x_n = a_n has been precomputed. We choose the first x_n, x0, to equal 2^7
        // because all larger powers are larger than MAX_NATURAL_EXPONENT, and therefore not present in the
        // decomposition.
        // At the end of this process we will have the product of all e^x_n = a_n that apply, and the remainder of this
        // decomposition, which will be lower than the smallest x_n.
        // exp(x) = k_0 * a_0 * k_1 * a_1 * ... + k_n * a_n * exp(remainder), where each k_n equals either 0 or 1.
        // We mutate x by subtracting x_n, making it the remainder of the decomposition.

        // The first two a_n (e^(2^7) and e^(2^6)) are too large if stored as 18 decimal numbers, and could cause
        // intermediate overflows. Instead we store them as plain integers, with 0 decimals.
        // Additionally, x0 + x1 is larger than MAX_NATURAL_EXPONENT, which means they will not both be present in the
        // decomposition.

        // For each x_n, we test if that term is present in the decomposition (if x is larger than it), and if so deduct
        // it and compute the accumulated product.

        int256 firstAN;
        unchecked {
            if (x >= x0) {
                x -= x0;
                firstAN = a0;
            } else if (x >= x1) {
                x -= x1;
                firstAN = a1;
            } else {
                firstAN = 1; // One with no decimal places
            }

            // We now transform x into a 20 decimal fixed point number, to have enhanced precision when computing the
            // smaller terms.
            x *= 100;
        }

        // `product` is the accumulated product of all a_n (except a0 and a1), which starts at 20 decimal fixed point
        // one. Recall that fixed point multiplication requires dividing by ONE_20.
        int256 product = ONE_20;

        unchecked {
            if (x >= x2) {
                x -= x2;
                product = (product * a2) / ONE_20;
            }
            if (x >= x3) {
                x -= x3;
                product = (product * a3) / ONE_20;
            }
            if (x >= x4) {
                x -= x4;
                product = (product * a4) / ONE_20;
            }
            if (x >= x5) {
                x -= x5;
                product = (product * a5) / ONE_20;
            }
            if (x >= x6) {
                x -= x6;
                product = (product * a6) / ONE_20;
            }
            if (x >= x7) {
                x -= x7;
                product = (product * a7) / ONE_20;
            }
            if (x >= x8) {
                x -= x8;
                product = (product * a8) / ONE_20;
            }
            if (x >= x9) {
                x -= x9;
                product = (product * a9) / ONE_20;
            }
        }

        // x10 and x11 are unnecessary here since we have high enough precision already.

        // Now we need to compute e^x, where x is small (in particular, it is smaller than x9). We use the Taylor series
        // expansion for e^x: 1 + x + (x^2 / 2!) + (x^3 / 3!) + ... + (x^n / n!).

        int256 seriesSum = ONE_20; // The initial one in the sum, with 20 decimal places.
        int256 term; // Each term in the sum, where the nth term is (x^n / n!).

        // The first term is simply x.
        term = x;
        unchecked {
            seriesSum += term;

            // Each term (x^n / n!) equals the previous one times x, divided by n. Since x is a fixed point number,
            // multiplying by it requires dividing by ONE_20, but dividing by the non-fixed point n values does not.

            term = ((term * x) / ONE_20) / 2;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 3;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 4;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 5;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 6;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 7;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 8;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 9;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 10;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 11;
            seriesSum += term;

            term = ((term * x) / ONE_20) / 12;
            seriesSum += term;

            // 12 Taylor terms are sufficient for 18 decimal precision.

            // We now have the first a_n (with no decimals), and the product of all other a_n present, and the Taylor
            // approximation of the exponentiation of the remainder (both with 20 decimals). All that remains is to multiply
            // all three (one 20 decimal fixed point multiplication, dividing by ONE_20, and one integer multiplication),
            // and then drop two digits to return an 18 decimal value.

            int256 result = (((product * seriesSum) / ONE_20) * firstAN) / 100;

            // We avoid using recursion here because zkSync doesn't support it.
            return negativeExponent ? (ONE_18 * ONE_18) / result : result;
        }
    }

    /// @dev Logarithm (log(arg, base), with signed 18 decimal fixed point base and argument.
    function log(int256 arg, int256 base) internal pure returns (int256) {
        // This performs a simple base change: log(arg, base) = ln(arg) / ln(base).

        // Both logBase and logArg are computed as 36 decimal fixed point numbers, either by using ln_36, or by
        // upscaling.

        int256 logBase;
        unchecked {
            if (LN_36_LOWER_BOUND < base && base < LN_36_UPPER_BOUND) {
                logBase = _ln_36(base);
            } else {
                logBase = _ln(base) * ONE_18;
            }
        }

        int256 logArg;
        unchecked {
            if (LN_36_LOWER_BOUND < arg && arg < LN_36_UPPER_BOUND) {
                logArg = _ln_36(arg);
            } else {
                logArg = _ln(arg) * ONE_18;
            }

            // When dividing, we multiply by ONE_18 to arrive at a result with 18 decimal places
            return (logArg * ONE_18) / logBase;
        }
    }

    /// @dev Natural logarithm (ln(a)) with signed 18 decimal fixed point argument.
    function ln(int256 a) internal pure returns (int256) {
        // The real natural logarithm is not defined for negative numbers or zero.
        if (a <= 0) {
            revert OutOfBounds();
        }
        if (LN_36_LOWER_BOUND < a && a < LN_36_UPPER_BOUND) {
            unchecked {
                return _ln_36(a) / ONE_18;
            }
        } else {
            return _ln(a);
        }
    }

    /// @dev Internal natural logarithm (ln(a)) with signed 18 decimal fixed point argument.
    function _ln(int256 a) private pure returns (int256) {
        // We avoid using recursion here because zkSync doesn't support it.
        bool negativeExponent = false;

        if (a < ONE_18) {
            // Since ln(a^k) = k * ln(a), we can compute ln(a) as ln(a) = ln((1/a)^(-1)) = - ln((1/a)). If a is less
            // than one, 1/a will be greater than one, so in this case we compute ln(1/a) and negate the final result.
            unchecked {
                a = (ONE_18 * ONE_18) / a;
            }
            negativeExponent = true;
        }

        // First, we use the fact that ln^(a * b) = ln(a) + ln(b) to decompose ln(a) into a sum of powers of two, which
        // we call x_n, where x_n == 2^(7 - n), which are the natural logarithm of precomputed quantities a_n (that is,
        // ln(a_n) = x_n). We choose the first x_n, x0, to equal 2^7 because the exponential of all larger powers cannot
        // be represented as 18 fixed point decimal numbers in 256 bits, and are therefore larger than a.
        // At the end of this process we will have the sum of all x_n = ln(a_n) that apply, and the remainder of this
        // decomposition, which will be lower than the smallest a_n.
        // ln(a) = k_0 * x_0 + k_1 * x_1 + ... + k_n * x_n + ln(remainder), where each k_n equals either 0 or 1.
        // We mutate a by subtracting a_n, making it the remainder of the decomposition.

        // For reasons related to how `exp` works, the first two a_n (e^(2^7) and e^(2^6)) are not stored as fixed point
        // numbers with 18 decimals, but instead as plain integers with 0 decimals, so we need to multiply them by
        // ONE_18 to convert them to fixed point.
        // For each a_n, we test if that term is present in the decomposition (if a is larger than it), and if so divide
        // by it and compute the accumulated sum.

        int256 sum = 0;
        unchecked {
            if (a >= a0 * ONE_18) {
                a /= a0; // Integer, not fixed point division
                sum += x0;
            }

            if (a >= a1 * ONE_18) {
                a /= a1; // Integer, not fixed point division
                sum += x1;
            }

            // All other a_n and x_n are stored as 20 digit fixed point numbers, so we convert the sum and a to this format.
            sum *= 100;
            a *= 100;

            // Because further a_n are  20 digit fixed point numbers, we multiply by ONE_20 when dividing by them.

            if (a >= a2) {
                a = (a * ONE_20) / a2;
                sum += x2;
            }

            if (a >= a3) {
                a = (a * ONE_20) / a3;
                sum += x3;
            }

            if (a >= a4) {
                a = (a * ONE_20) / a4;
                sum += x4;
            }

            if (a >= a5) {
                a = (a * ONE_20) / a5;
                sum += x5;
            }

            if (a >= a6) {
                a = (a * ONE_20) / a6;
                sum += x6;
            }

            if (a >= a7) {
                a = (a * ONE_20) / a7;
                sum += x7;
            }

            if (a >= a8) {
                a = (a * ONE_20) / a8;
                sum += x8;
            }

            if (a >= a9) {
                a = (a * ONE_20) / a9;
                sum += x9;
            }

            if (a >= a10) {
                a = (a * ONE_20) / a10;
                sum += x10;
            }

            if (a >= a11) {
                a = (a * ONE_20) / a11;
                sum += x11;
            }
        }

        // a is now a small number (smaller than a_11, which roughly equals 1.06). This means we can use a Taylor series
        // that converges rapidly for values of `a` close to one - the same one used in ln_36.
        // Let z = (a - 1) / (a + 1).
        // ln(a) = 2 * (z + z^3 / 3 + z^5 / 5 + z^7 / 7 + ... + z^(2 * n + 1) / (2 * n + 1))

        // Recall that 20 digit fixed point division requires multiplying by ONE_20, and multiplication requires
        // division by ONE_20.
        unchecked {
            int256 z = ((a - ONE_20) * ONE_20) / (a + ONE_20);
            int256 z_squared = (z * z) / ONE_20;

            // num is the numerator of the series: the z^(2 * n + 1) term
            int256 num = z;

            // seriesSum holds the accumulated sum of each term in the series, starting with the initial z
            int256 seriesSum = num;

            // In each step, the numerator is multiplied by z^2
            num = (num * z_squared) / ONE_20;
            seriesSum += num / 3;

            num = (num * z_squared) / ONE_20;
            seriesSum += num / 5;

            num = (num * z_squared) / ONE_20;
            seriesSum += num / 7;

            num = (num * z_squared) / ONE_20;
            seriesSum += num / 9;

            num = (num * z_squared) / ONE_20;
            seriesSum += num / 11;

            // 6 Taylor terms are sufficient for 36 decimal precision.

            // Finally, we multiply by 2 (non fixed point) to compute ln(remainder)
            seriesSum *= 2;

            // We now have the sum of all x_n present, and the Taylor approximation of the logarithm of the remainder (both
            // with 20 decimals). All that remains is to sum these two, and then drop two digits to return a 18 decimal
            // value.

            int256 result = (sum + seriesSum) / 100;

            // We avoid using recursion here because zkSync doesn't support it.
            return negativeExponent ? -result : result;
        }
    }

    /**
     * @dev Internal high precision (36 decimal places) natural logarithm (ln(x)) with signed 18 decimal fixed point argument,
     * for x close to one.
     *
     * Should only be used if x is between LN_36_LOWER_BOUND and LN_36_UPPER_BOUND.
     */
    function _ln_36(int256 x) private pure returns (int256) {
        // Since ln(1) = 0, a value of x close to one will yield a very small result, which makes using 36 digits
        // worthwhile.

        // First, we transform x to a 36 digit fixed point value.
        unchecked {
            x *= ONE_18;

            // We will use the following Taylor expansion, which converges very rapidly. Let z = (x - 1) / (x + 1).
            // ln(x) = 2 * (z + z^3 / 3 + z^5 / 5 + z^7 / 7 + ... + z^(2 * n + 1) / (2 * n + 1))

            // Recall that 36 digit fixed point division requires multiplying by ONE_36, and multiplication requires
            // division by ONE_36.
            int256 z = ((x - ONE_36) * ONE_36) / (x + ONE_36);
            int256 z_squared = (z * z) / ONE_36;

            // num is the numerator of the series: the z^(2 * n + 1) term
            int256 num = z;

            // seriesSum holds the accumulated sum of each term in the series, starting with the initial z
            int256 seriesSum = num;

            // In each step, the numerator is multiplied by z^2
            num = (num * z_squared) / ONE_36;
            seriesSum += num / 3;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 5;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 7;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 9;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 11;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 13;

            num = (num * z_squared) / ONE_36;
            seriesSum += num / 15;

            // 8 Taylor terms are sufficient for 36 decimal precision.

            // All that remains is multiplying by 2 (non fixed point).
            return seriesSum * 2;
        }
    }
}

// pkg/solidity-utils/contracts/math/FixedPoint.sol

/// @notice Support 18-decimal fixed point arithmetic. All Vault calculations use this for high and uniform precision.
library FixedPoint {
    /// @notice Attempted division by zero.
    error ZeroDivision();

    // solhint-disable no-inline-assembly
    // solhint-disable private-vars-leading-underscore

    uint256 internal constant ONE = 1e18; // 18 decimal places
    uint256 internal constant TWO = 2 * ONE;
    uint256 internal constant FOUR = 4 * ONE;
    uint256 internal constant MAX_POW_RELATIVE_ERROR = 10000; // 10^(-14)

    function mulDown(uint256 a, uint256 b) internal pure returns (uint256) {
        // Multiplication overflow protection is provided by Solidity 0.8.x.
        uint256 product = a * b;

        return product / ONE;
    }

    function mulUp(uint256 a, uint256 b) internal pure returns (uint256 result) {
        // Multiplication overflow protection is provided by Solidity 0.8.x.
        uint256 product = a * b;

        // Equivalent to:
        // result = product == 0 ? 0 : ((product - 1) / FixedPoint.ONE) + 1
        assembly ("memory-safe") {
            result := mul(iszero(iszero(product)), add(div(sub(product, 1), ONE), 1))
        }
    }

    function divDown(uint256 a, uint256 b) internal pure returns (uint256) {
        // Solidity 0.8 reverts with a Panic code (0x11) if the multiplication overflows.
        uint256 aInflated = a * ONE;

        // Solidity 0.8 reverts with a "Division by Zero" Panic code (0x12) if b is zero
        return aInflated / b;
    }

    function divUp(uint256 a, uint256 b) internal pure returns (uint256 result) {
        return mulDivUp(a, ONE, b);
    }

    /// @dev Return (a * b) / c, rounding up.
    function mulDivUp(uint256 a, uint256 b, uint256 c) internal pure returns (uint256 result) {
        // This check is required because Yul's `div` doesn't revert on c==0.
        if (c == 0) {
            revert ZeroDivision();
        }

        // Multiple overflow protection is done by Solidity 0.8.x.
        uint256 product = a * b;

        // The traditional divUp formula is:
        // divUp(x, y) := (x + y - 1) / y
        // To avoid intermediate overflow in the addition, we distribute the division and get:
        // divUp(x, y) := (x - 1) / y + 1
        // Note that this requires x != 0, if x == 0 then the result is zero
        //
        // Equivalent to:
        // result = a == 0 ? 0 : (a * b - 1) / c + 1
        assembly ("memory-safe") {
            result := mul(iszero(iszero(product)), add(div(sub(product, 1), c), 1))
        }
    }

    /**
     * @dev Version of divUp when the input is raw (i.e., already "inflated"). For instance,
     * invariant * invariant (36 decimals) vs. invariant.mulDown(invariant) (18 decimal FP).
     * This can occur in calculations with many successive multiplications and divisions, and
     * we want to minimize the number of operations by avoiding unnecessary scaling by ONE.
     */
    function divUpRaw(uint256 a, uint256 b) internal pure returns (uint256 result) {
        // This check is required because Yul's `div` doesn't revert on b==0.
        if (b == 0) {
            revert ZeroDivision();
        }

        // Equivalent to:
        // result = a == 0 ? 0 : 1 + (a - 1) / b
        assembly ("memory-safe") {
            result := mul(iszero(iszero(a)), add(1, div(sub(a, 1), b)))
        }
    }

    /**
     * @dev Returns x^y, assuming both are fixed point numbers, rounding down. The result is guaranteed to not be above
     * the true value (that is, the error function expected - actual is always positive).
     */
    function powDown(uint256 x, uint256 y) internal pure returns (uint256) {
        // Optimize for when y equals 1.0, 2.0 or 4.0, as those are very simple to implement and occur often in 50/50
        // and 80/20 Weighted Pools
        if (y == ONE) {
            return x;
        } else if (y == TWO) {
            return mulDown(x, x);
        } else if (y == FOUR) {
            uint256 square = mulDown(x, x);
            return mulDown(square, square);
        } else {
            uint256 raw = LogExpMath.pow(x, y);
            uint256 maxError = mulUp(raw, MAX_POW_RELATIVE_ERROR) + 1;

            if (raw < maxError) {
                return 0;
            } else {
                unchecked {
                    return raw - maxError;
                }
            }
        }
    }

    /**
     * @dev Returns x^y, assuming both are fixed point numbers, rounding up. The result is guaranteed to not be below
     * the true value (that is, the error function expected - actual is always negative).
     */
    function powUp(uint256 x, uint256 y) internal pure returns (uint256) {
        // Optimize for when y equals 1.0, 2.0 or 4.0, as those are very simple to implement and occur often in 50/50
        // and 80/20 Weighted Pools
        if (y == ONE) {
            return x;
        } else if (y == TWO) {
            return mulUp(x, x);
        } else if (y == FOUR) {
            uint256 square = mulUp(x, x);
            return mulUp(square, square);
        } else {
            uint256 raw = LogExpMath.pow(x, y);
            uint256 maxError = mulUp(raw, MAX_POW_RELATIVE_ERROR) + 1;

            return raw + maxError;
        }
    }

    /**
     * @dev Returns the complement of a value (1 - x), capped to 0 if x is larger than 1.
     *
     * Useful when computing the complement for values with some level of relative error, as it strips this error and
     * prevents intermediate negative values.
     */
    function complement(uint256 x) internal pure returns (uint256 result) {
        // Equivalent to:
        // result = (x < ONE) ? (ONE - x) : 0
        assembly ("memory-safe") {
            result := mul(lt(x, ONE), sub(ONE, x))
        }
    }
}

// pkg/solidity-utils/contracts/math/StableMath.sol

/**
 * @notice Stable Pool math library based on Curve's `StableSwap`.
 * @dev See https://docs.curve.fi/references/whitepapers/stableswap/
 *
 * For security reasons, to help ensure that for all possible "round trip" paths the caller always receives the same
 * or fewer tokens than supplied, we have used precise math (i.e., '*', '/' vs. FixedPoint) whenever possible, and
 * chosen the rounding direction to favor the protocol elsewhere.
 *
 * `computeInvariant` does not use the rounding direction from `IBasePool`, effectively always rounding down to match
 * the Curve implementation.
 */
library StableMath {
    using FixedPoint for uint256;

    // Some variables have non mixed case names (e.g. P_D) that relate to the mathematical derivations.
    // solhint-disable private-vars-leading-underscore, var-name-mixedcase

    /// @notice The iterations to calculate the invariant didn't converge.
    error StableInvariantDidNotConverge();

    /// @notice The iterations to calculate the balance didn't converge.
    error StableComputeBalanceDidNotConverge();

    // The max token count is limited by the math, and is less than the Vault's maximum.
    uint256 public constant MAX_STABLE_TOKENS = 5;

    uint256 internal constant MIN_AMP = 1;
    uint256 internal constant MAX_AMP = 50000;
    uint256 internal constant AMP_PRECISION = 1e3;

    // Invariant growth limit: non-proportional add cannot cause the invariant to increase by more than this ratio.
    uint256 internal constant MIN_INVARIANT_RATIO = 60e16; // 60%
    // Invariant shrink limit: non-proportional remove cannot cause the invariant to decrease by less than this ratio.
    uint256 internal constant MAX_INVARIANT_RATIO = 500e16; // 500%

    // About swap fees on add and remove liquidity:
    // Any add or remove that is not perfectly balanced (e.g. all single token operations) is mathematically
    // equivalent to a perfectly balanced add or remove followed by a series of swaps. Since these swaps would charge
    // swap fees, it follows that unbalanced adds and removes should as well.
    //
    // On these operations, we split the token amounts in 'taxable' and 'non-taxable' portions, where the 'taxable' part
    // is the one to which swap fees are applied.

    // See: https://github.com/curvefi/curve-contract/blob/b0bbf77f8f93c9c5f4e415bce9cd71f0cdee960e/contracts/pool-templates/base/SwapTemplateBase.vy#L206
    // solhint-disable-previous-line max-line-length

    /**
     * @notice Computes the invariant given the current balances.
     * @dev It uses the Newton-Raphson approximation. The amplification parameter is given by: A n^(n-1).
     * There is no closed-form solution, so the calculation is iterative and may revert.
     *
     * @param amplificationParameter The current amplification parameter
     * @param balances The current balances
     * @return invariant The calculated invariant of the pool
     */
    function computeInvariant(
        uint256 amplificationParameter,
        uint256[] memory balances
    ) internal pure returns (uint256) {
        /**********************************************************************************************
        // invariant                                                                                 //
        // D = invariant                                                  D^(n+1)                    //
        // A = amplification coefficient      A  n^n S + D = A D n^n + -----------                   //
        // S = sum of balances                                             n^n P                     //
        // P = product of balances                                                                   //
        // n = number of tokens                                                                      //
        **********************************************************************************************/

        uint256 sum = 0; // S in the Curve version
        uint256 numTokens = balances.length;
        for (uint256 i = 0; i < numTokens; ++i) {
            sum = sum + balances[i];
        }
        if (sum == 0) {
            return 0;
        }

        uint256 prevInvariant; // Dprev in the Curve version
        uint256 invariant = sum; // D in the Curve version
        uint256 ampTimesTotal = amplificationParameter * numTokens; // Ann in the Curve version

        for (uint256 i = 0; i < 255; ++i) {
            uint256 D_P = invariant;
            for (uint256 j = 0; j < numTokens; ++j) {
                D_P = (D_P * invariant) / (balances[j] * numTokens);
            }

            prevInvariant = invariant;

            invariant =
                ((((ampTimesTotal * sum) / AMP_PRECISION) + (D_P * numTokens)) * invariant) /
                ((((ampTimesTotal - AMP_PRECISION) * invariant) / AMP_PRECISION) + ((numTokens + 1) * D_P));

            unchecked {
                // We are explicitly checking the magnitudes here, so can use unchecked math.
                if (invariant > prevInvariant) {
                    if (invariant - prevInvariant <= 1) {
                        return invariant;
                    }
                } else if (prevInvariant - invariant <= 1) {
                    return invariant;
                }
            }
        }

        revert StableInvariantDidNotConverge();
    }

    /**
     * @notice Computes the required `amountOut` of tokenOut, for `tokenAmountIn` of tokenIn.
     * @dev The calculation uses the Newton-Raphson approximation. The amplification parameter is given by: A n^(n-1).
     * @param amplificationParameter The current amplification factor
     * @param balances The current pool balances
     * @param tokenIndexIn The index of tokenIn
     * @param tokenIndexOut The index of tokenOut
     * @param tokenAmountIn The exact amount of tokenIn specified for the swap
     * @param invariant The current invariant
     * @return amountOut The calculated amount of tokenOut required for the swap
     */
    function computeOutGivenExactIn(
        uint256 amplificationParameter,
        uint256[] memory balances,
        uint256 tokenIndexIn,
        uint256 tokenIndexOut,
        uint256 tokenAmountIn,
        uint256 invariant
    ) internal pure returns (uint256) {
        /**************************************************************************************************************
        // outGivenExactIn token x for y - polynomial equation to solve                                              //
        // ay = amount out to calculate                                                                              //
        // by = balance token out                                                                                    //
        // y = by - ay (finalBalanceOut)                                                                             //
        // D = invariant                                               D                     D^(n+1)                 //
        // A = amplification coefficient               y^2 + ( S + ----------  - D) * y -  ------------- = 0         //
        // n = number of tokens                                    (A * n^n)               A * n^2n * P              //
        // S = sum of final balances but y                                                                           //
        // P = product of final balances but y                                                                       //
        **************************************************************************************************************/

        balances[tokenIndexIn] += tokenAmountIn;

        // `computeBalance` rounds up.
        uint256 finalBalanceOut = computeBalance(amplificationParameter, balances, invariant, tokenIndexOut);

        // No need to use checked arithmetic since `tokenAmountIn` was actually added to the same balance right before
        // calling `computeBalance`, which doesn't alter the balances array.
        unchecked {
            balances[tokenIndexIn] -= tokenAmountIn;
        }

        // Amount out, so we round down overall.
        return balances[tokenIndexOut] - finalBalanceOut - 1;
    }

    /**
     * @notice Computes the required `amountIn` of tokenIn, for `tokenAmountOut` of tokenOut.
     * @dev The calculation uses the Newton-Raphson approximation. The amplification parameter is given by: A n^(n-1).
     * @param amplificationParameter The current amplification factor
     * @param balances The current pool balances
     * @param tokenIndexIn The index of tokenIn
     * @param tokenIndexOut The index of tokenOut
     * @param tokenAmountOut The exact amount of tokenOut specified for the swap
     * @param invariant The current invariant
     * @return amountIn The calculated amount of tokenIn required for the swap
     */
    function computeInGivenExactOut(
        uint256 amplificationParameter,
        uint256[] memory balances,
        uint256 tokenIndexIn,
        uint256 tokenIndexOut,
        uint256 tokenAmountOut,
        uint256 invariant
    ) internal pure returns (uint256) {
        /**************************************************************************************************************
        // inGivenExactOut token x for y - polynomial equation to solve                                              //
        // ax = amount in to calculate                                                                               //
        // bx = balance token in                                                                                     //
        // x = bx + ax (finalBalanceIn)                                                                              //
        // D = invariant                                                D                     D^(n+1)                //
        // A = amplification coefficient               x^2 + ( S + ----------  - D) * x -  ------------- = 0         //
        // n = number of tokens                                     (A * n^n)               A * n^2n * P             //
        // S = sum of final balances but x                                                                           //
        // P = product of final balances but x                                                                       //
        **************************************************************************************************************/

        balances[tokenIndexOut] -= tokenAmountOut;

        // `computeBalance` rounds up.
        uint256 finalBalanceIn = computeBalance(amplificationParameter, balances, invariant, tokenIndexIn);

        // No need to use checked arithmetic since `tokenAmountOut` was actually subtracted from the same balance right
        // before calling `computeBalance`, which doesn't alter the balances array.
        unchecked {
            balances[tokenIndexOut] += tokenAmountOut;
        }

        // Amount in, so we round up overall.
        return finalBalanceIn - balances[tokenIndexIn] + 1;
    }

    /**
     * @notice Calculate the balance of a given token (at tokenIndex), given all other balances and the invariant.
     * @dev Rounds result up overall. There is no closed-form solution, so the calculation is iterative and may revert.
     * @param amplificationParameter The current amplification factor
     * @param balances The current pool balances
     * @param invariant The current invariant
     * @param tokenIndex The index of the token balance we are calculating
     * @return tokenBalance The adjusted balance of the token at `tokenIn` that matches the given invariant
     */
    function computeBalance(
        uint256 amplificationParameter,
        uint256[] memory balances,
        uint256 invariant,
        uint256 tokenIndex
    ) internal pure returns (uint256) {
        uint256 numTokens = balances.length;
        uint256 ampTimesTotal = amplificationParameter * numTokens;
        uint256 sum = balances[0];
        uint256 P_D = balances[0] * numTokens;
        for (uint256 j = 1; j < numTokens; ++j) {
            P_D = (P_D * balances[j] * numTokens) / invariant;
            sum = sum + balances[j];
        }
        sum = sum - balances[tokenIndex];

        // Use divUpRaw with inv2, as it is a "raw" 36 decimal value.
        uint256 inv2 = invariant * invariant;
        // We remove the balance from c by multiplying it.
        uint256 c = (inv2 * AMP_PRECISION).divUpRaw(ampTimesTotal * P_D) * balances[tokenIndex];
        uint256 b = sum + ((invariant * AMP_PRECISION) / ampTimesTotal);
        // We iterate to find the balance.
        uint256 prevTokenBalance = 0;
        // We multiply the first iteration outside the loop with the invariant to set the value of the
        // initial approximation.
        uint256 tokenBalance = (inv2 + c).divUpRaw(invariant + b);

        for (uint256 i = 0; i < 255; ++i) {
            prevTokenBalance = tokenBalance;

            // Use divUpRaw with tokenBalance, as it is a "raw" 36 decimal value.
            tokenBalance = ((tokenBalance * tokenBalance) + c).divUpRaw((tokenBalance * 2) + b - invariant);

            // We are explicitly checking the magnitudes here, so can use unchecked math.
            unchecked {
                if (tokenBalance > prevTokenBalance) {
                    if (tokenBalance - prevTokenBalance <= 1) {
                        return tokenBalance;
                    }
                } else if (prevTokenBalance - tokenBalance <= 1) {
                    return tokenBalance;
                }
            }
        }

        revert StableComputeBalanceDidNotConverge();
    }
}

```


## ./pkg/solidity-utils/contracts/solmate/CREATE3.sol

```solidity
// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity ^0.8.24;

// pkg/solidity-utils/contracts/solmate/Bytes32AddressLib.sol

/**
 * @notice Library for converting between addresses and bytes32 values.
 * @author Solmate (https://github.com/transmissions11/solmate/blob/main/src/utils/Bytes32AddressLib.sol)
 * @dev Used in CREATE3 contract deployment.
 */
library Bytes32AddressLib {
    function fromLast20Bytes(bytes32 bytesValue) internal pure returns (address) {
        return address(uint160(uint256(bytesValue)));
    }

    function fillLast12Bytes(address addressValue) internal pure returns (bytes32) {
        return bytes32(bytes20(addressValue));
    }
}

// pkg/solidity-utils/contracts/solmate/CREATE3.sol

/**
 * @notice Deploy to deterministic addresses without an initcode factor.
 * @author Solmate (https://github.com/transmissions11/solmate/blob/main/src/utils/CREATE3.sol)
 * @dev Modified from 0xSequence (https://github.com/0xSequence/create3/blob/master/contracts/Create3.sol)
 * Also avoids dependence on a particular deployer account, and allows for more secure "salt mining" of addresses,
 * vs. web-based vanity address mining.
 */
library CREATE3 {
    using Bytes32AddressLib for bytes32;
    // solhint-disable no-inline-assembly

    //--------------------------------------------------------------------------------//
    // Opcode     | Opcode + Arguments    | Description      | Stack View             //
    //--------------------------------------------------------------------------------//
    // 0x36       |  0x36                 | CALLDATASIZE     | size                   //
    // 0x3d       |  0x3d                 | RETURNDATASIZE   | 0 size                 //
    // 0x3d       |  0x3d                 | RETURNDATASIZE   | 0 0 size               //
    // 0x37       |  0x37                 | CALLDATACOPY     |                        //
    // 0x36       |  0x36                 | CALLDATASIZE     | size                   //
    // 0x3d       |  0x3d                 | RETURNDATASIZE   | 0 size                 //
    // 0x34       |  0x34                 | CALLVALUE        | value 0 size           //
    // 0xf0       |  0xf0                 | CREATE           | newContract            //
    //--------------------------------------------------------------------------------//
    // Opcode     | Opcode + Arguments    | Description      | Stack View             //
    //--------------------------------------------------------------------------------//
    // 0x67       |  0x67XXXXXXXXXXXXXXXX | PUSH8 bytecode   | bytecode               //
    // 0x3d       |  0x3d                 | RETURNDATASIZE   | 0 bytecode             //
    // 0x52       |  0x52                 | MSTORE           |                        //
    // 0x60       |  0x6008               | PUSH1 08         | 8                      //
    // 0x60       |  0x6018               | PUSH1 18         | 24 8                   //
    // 0xf3       |  0xf3                 | RETURN           |                        //
    //--------------------------------------------------------------------------------//
    bytes internal constant _PROXY_BYTECODE = hex"67_36_3d_3d_37_36_3d_34_f0_3d_52_60_08_60_18_f3";

    bytes32 internal constant _PROXY_BYTECODE_HASH = keccak256(_PROXY_BYTECODE);

    function deploy(bytes32 salt, bytes memory creationCode, uint256 value) internal returns (address deployed) {
        bytes memory proxyChildBytecode = _PROXY_BYTECODE;

        address proxy;
        /// @solidity memory-safe-assembly
        assembly {
            // Deploy a new contract with our pre-made bytecode via CREATE2.
            // We start 32 bytes into the code to avoid copying the byte length.
            proxy := create2(0, add(proxyChildBytecode, 32), mload(proxyChildBytecode), salt)
        }
        require(proxy != address(0), "DEPLOYMENT_FAILED");

        deployed = getDeployed(salt);
        (bool success, ) = proxy.call{ value: value }(creationCode);
        require(success && deployed.code.length != 0, "INITIALIZATION_FAILED");
    }

    function getDeployed(bytes32 salt) internal view returns (address) {
        return getDeployed(salt, address(this));
    }

    function getDeployed(bytes32 salt, address creator) internal pure returns (address) {
        address proxy = keccak256(
            abi.encodePacked(
                // Prefix:
                bytes1(0xFF),
                // Creator:
                creator,
                // Salt:
                salt,
                // Bytecode hash:
                _PROXY_BYTECODE_HASH
            )
        ).fromLast20Bytes();

        return
            keccak256(
                abi.encodePacked(
                    // 0xd6 = 0xc0 (short RLP prefix) + 0x16 (length of: 0x94 ++ proxy ++ 0x01)
                    // 0x94 = 0x80 + 0x14 (0x14 = the length of an address, 20 bytes, in hex)
                    hex"d6_94",
                    proxy,
                    hex"01" // Nonce of the proxy contract (1)
                )
            ).fromLast20Bytes();
    }
}

```


## ./pkg/solidity-utils/contracts/solmate/Bytes32AddressLib.sol

```solidity
// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity ^0.8.24;

// pkg/solidity-utils/contracts/solmate/Bytes32AddressLib.sol

/**
 * @notice Library for converting between addresses and bytes32 values.
 * @author Solmate (https://github.com/transmissions11/solmate/blob/main/src/utils/Bytes32AddressLib.sol)
 * @dev Used in CREATE3 contract deployment.
 */
library Bytes32AddressLib {
    function fromLast20Bytes(bytes32 bytesValue) internal pure returns (address) {
        return address(uint160(uint256(bytesValue)));
    }

    function fillLast12Bytes(address addressValue) internal pure returns (bytes32) {
        return bytes32(bytes20(addressValue));
    }
}

```


## ./pkg/solidity-utils/contracts/helpers/EVMCallModeHelpers.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/solidity-utils/contracts/helpers/EVMCallModeHelpers.sol

/// @notice Library used to check whether the current operation was initiated through a static call.
library EVMCallModeHelpers {
    /// @notice A state-changing transaction was initiated in a context that only allows static calls.
    error NotStaticCall();

    /**
     * @dev Detects whether the current transaction is a static call.
     * A static call is one where `tx.origin` equals 0x0 for most implementations.
     * See this tweet for a table on how transaction parameters are set on different platforms:
     * https://twitter.com/0xkarmacoma/status/1493380279309717505
     *
     * Solidity eth_call reference docs are here: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_call
     */
    function isStaticCall() internal view returns (bool) {
        return tx.origin == address(0);
        // solhint-disable-previous-line avoid-tx-origin
    }
}

```


## ./pkg/solidity-utils/contracts/helpers/PackedTokenBalance.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/solidity-utils/contracts/helpers/PackedTokenBalance.sol

/**
 * @notice This library represents a data structure for packing a token's current raw and derived balances. A derived
 * balance can be the "last" live balance scaled18 of the raw token, or the balance of the wrapped version of the
 * token in a vault buffer, among others.
 *
 * @dev We could use a Solidity struct to pack balance values together in a single storage slot, but unfortunately
 * Solidity only allows for structs to live in either storage, calldata or memory. Because a memory struct still takes
 * up a slot in the stack (to store its memory location), and because the entire balance fits in a single stack slot
 * (two 128 bit values), using memory is strictly less gas performant. Therefore, we do manual packing and unpacking.
 *
 * We could also use custom types now, but given the simplicity here, and the existing EnumerableMap type, it seemed
 * easier to leave it as a bytes32.
 */
library PackedTokenBalance {
    // The 'rawBalance' portion of the balance is stored in the least significant 128 bits of a 256 bit word, while the
    // The 'derivedBalance' part uses the remaining 128 bits.
    uint256 private constant _MAX_BALANCE = 2 ** (128) - 1;

    /// @notice One of the balances is above the maximum value that can be stored.
    error BalanceOverflow();

    function getBalanceRaw(bytes32 balance) internal pure returns (uint256) {
        return uint256(balance) & _MAX_BALANCE;
    }

    function getBalanceDerived(bytes32 balance) internal pure returns (uint256) {
        return uint256(balance >> 128) & _MAX_BALANCE;
    }

    /// @dev Sets only the raw balance of balances and returns the new bytes32 balance.
    function setBalanceRaw(bytes32 balance, uint256 newBalanceRaw) internal pure returns (bytes32) {
        return toPackedBalance(newBalanceRaw, getBalanceDerived(balance));
    }

    /// @dev Sets only the derived balance of balances and returns the new bytes32 balance.
    function setBalanceDerived(bytes32 balance, uint256 newBalanceDerived) internal pure returns (bytes32) {
        return toPackedBalance(getBalanceRaw(balance), newBalanceDerived);
    }

    /// @dev Validates the size of `balanceRaw` and `balanceDerived`, then returns a packed balance bytes32.
    function toPackedBalance(uint256 balanceRaw, uint256 balanceDerived) internal pure returns (bytes32) {
        if (balanceRaw > _MAX_BALANCE || balanceDerived > _MAX_BALANCE) {
            revert BalanceOverflow();
        }

        return _pack(balanceRaw, balanceDerived);
    }

    /// @dev Decode and fetch both balances.
    function fromPackedBalance(bytes32 balance) internal pure returns (uint256 balanceRaw, uint256 balanceDerived) {
        return (getBalanceRaw(balance), getBalanceDerived(balance));
    }

    /// @dev Packs two uint128 values into a packed balance bytes32. It does not check balance sizes.
    function _pack(uint256 leastSignificant, uint256 mostSignificant) private pure returns (bytes32) {
        return bytes32((mostSignificant << 128) + leastSignificant);
    }
}

```


## ./pkg/solidity-utils/contracts/helpers/CodeDeployer.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/solidity-utils/contracts/helpers/CodeDeployer.sol

/**
 * @dev Library used to deploy contracts with specific code. This can be used for long-term storage of immutable data as
 * contract code, which can be retrieved via the `extcodecopy` opcode.
 */
library CodeDeployer {
    error CodeDeploymentFailed();

    // During contract construction, the full code supplied exists as code, and can be accessed via `codesize` and
    // `codecopy`. This is not the contract's final code however: whatever the constructor returns is what will be
    // stored as its code.
    //
    // We use this mechanism to have a simple constructor that stores whatever is appended to it. The following opcode
    // sequence corresponds to the creation code of the following equivalent Solidity contract, plus padding to make the
    // full code 32 bytes long:
    //
    // contract CodeDeployer {
    //     constructor() payable {
    //         uint256 size;
    //         assembly {
    //             size := sub(codesize(), 32) // size of appended data, as constructor is 32 bytes long
    //             codecopy(0, 32, size) // copy all appended data to memory at position 0
    //             return(0, size) // return appended data for it to be stored as code
    //         }
    //     }
    // }
    //
    // More specifically, it is composed of the following opcodes (plus padding):
    //
    // [1] PUSH1 0x20
    // [2] CODESIZE
    // [3] SUB
    // [4] DUP1
    // [6] PUSH1 0x20
    // [8] PUSH1 0x00
    // [9] CODECOPY
    // [11] PUSH1 0x00
    // [12] RETURN
    //
    // The padding is just the 0xfe sequence (invalid opcode). It is important as it lets us work in-place, avoiding
    // memory allocation and copying.

    bytes32 private constant _DEPLOYER_CREATION_CODE =
        0x602038038060206000396000f3fefefefefefefefefefefefefefefefefefefe;

    // Sometimes (e.g., when deploying the second or "B" half of the creation code in BaseSplitCodeFactory), we need to
    // protect the bare contract from being accidentally (or maliciously) executed, in case the bytes at the boundary
    // happen to be valid opcodes. It's especially dangerous if the bytes contained the selfdestruct opcode, which would
    // destroy the contract (and, if it's a factory, effectively disable it and prevent further pool creation).
    //
    // To guard against this, if the "preventExecution" flag is set, we prepend an invalid opcode to the contract,
    // to ensure that it cannot be executed, regardless of its content.
    //
    // This corresponds to the following contract:
    //
    // contract CodeDeployer {
    //     constructor() payable {
    //         uint256 size;
    //         assembly {
    //             mstore8(0, 0xfe) // store invalid opcode at position 0
    //             size := sub(codesize(), 32) // size of appended data, as constructor is 32 bytes long
    //             codecopy(1, 32, size) // copy all appended data to memory at position 1
    //             return(0, add(size, 1)) // return appended data (plus the extra byte) for it to be stored as code
    //         }
    //     }
    // }
    //
    // More specifically, it is composed of the following opcodes (plus padding, described above):
    //
    // [1] PUSH1 0xfe
    // [3] PUSH1 0x00
    // [4] MSTORE8
    // [6] PUSH1 0x20
    // [7] CODESIZE
    // [8] SUB
    // [9] DUP1
    // [11] PUSH1 0x20
    // [13] PUSH1 0x01
    // [14] CODECOPY
    // [16] PUSH1 0x01
    // [17] ADD
    // [19] PUSH1 0x00
    // [20] RETURN

    // solhint-disable max-line-length
    bytes32 private constant _PROTECTED_DEPLOYER_CREATION_CODE =
        0x60fe600053602038038060206001396001016000f3fefefefefefefefefefefe;

    /**
     * @dev Deploys a contract with `code` as its code, returning the destination address.
     * If preventExecution is set, prepend an invalid opcode to ensure the "contract" cannot be executed.
     * Rather than add a flag, we could simply always prepend the opcode, but there might be use cases where fidelity
     * is required.
     *
     * Reverts if deployment fails.
     */
    function deploy(bytes memory code, bool preventExecution) internal returns (address destination) {
        bytes32 deployerCreationCode = preventExecution ? _PROTECTED_DEPLOYER_CREATION_CODE : _DEPLOYER_CREATION_CODE;

        // We need to concatenate the deployer creation code and `code` in memory, but want to avoid copying all of
        // `code` (which could be quite long) into a new memory location. Therefore, we operate in-place using
        // assembly.

        // solhint-disable-next-line no-inline-assembly
        assembly {
            let codeLength := mload(code)

            // `code` is composed of length and data. We've already stored its length in `codeLength`, so we simply
            // replace it with the deployer creation code (which is exactly 32 bytes long).
            mstore(code, deployerCreationCode)

            // At this point, `code` now points to the deployer creation code immediately followed by `code`'s data
            // contents. This is exactly what the deployer expects to receive when created.
            destination := create(0, code, add(codeLength, 32))

            // Finally, we restore the original length in order to not mutate `code`.
            mstore(code, codeLength)
        }

        // The create opcode returns the zero address when contract creation fails, so we revert if this happens.
        if (destination == address(0)) {
            revert CodeDeploymentFailed();
        }
    }
}

```


## ./pkg/solidity-utils/contracts/helpers/RevertCodec.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/solidity-utils/contracts/helpers/RevertCodec.sol

// solhint-disable no-inline-assembly

/// @notice Support `quoteAndRevert`: a v2-style query which always reverts, and returns the result in the return data.
library RevertCodec {
    /**
     * @notice On success of the primary operation in a `quoteAndRevert`, this error is thrown with the return data.
     * @param result The result of the query operation
     */
    error Result(bytes result);

    /// @notice Handle the "reverted without a reason" case (i.e., no return data).
    error ErrorSelectorNotFound();

    function catchEncodedResult(bytes memory resultRaw) internal pure returns (bytes memory) {
        bytes4 errorSelector = RevertCodec.parseSelector(resultRaw);
        if (errorSelector != Result.selector) {
            // Bubble up error message if the revert reason is not the expected one.
            RevertCodec.bubbleUpRevert(resultRaw);
        }

        uint256 resultRawLength = resultRaw.length;
        assembly ("memory-safe") {
            resultRaw := add(resultRaw, 0x04) // Slice the sighash
            mstore(resultRaw, sub(resultRawLength, 4)) // Set proper length
        }

        return abi.decode(resultRaw, (bytes));
    }

    /// @dev Returns the first 4 bytes in an array, reverting if the length is < 4.
    function parseSelector(bytes memory callResult) internal pure returns (bytes4 errorSelector) {
        if (callResult.length < 4) {
            revert ErrorSelectorNotFound();
        }
        assembly ("memory-safe") {
            errorSelector := mload(add(callResult, 0x20)) // Load the first 4 bytes from data (skip length offset)
        }
    }

    /// @dev Taken from Openzeppelin's Address.
    function bubbleUpRevert(bytes memory returnData) internal pure {
        // Look for revert reason and bubble it up if present.
        if (returnData.length > 0) {
            // The easiest way to bubble the revert reason is using memory via assembly.

            assembly ("memory-safe") {
                let return_dataSize := mload(returnData)
                revert(add(32, returnData), return_dataSize)
            }
        } else {
            revert ErrorSelectorNotFound();
        }
    }
}

```


## ./pkg/solidity-utils/contracts/helpers/FactoryWidePauseWindow.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/solidity-utils/contracts/helpers/FactoryWidePauseWindow.sol

/**
 * @notice Base contract for v3 factories to support pause windows for pools based on the factory deployment time.
 * @dev Each pool deployment calls `getPauseWindowDuration` on the factory so that all Pools created by this factory
 * will share the same Pause Window end time, after which both old and new Pools will not be pausable.
 *
 * All pools are reversibly pausable until the pause window expires. Afterward, there is an additional buffer
 * period, set to the same duration as the Vault's buffer period. If a pool was paused, it will remain paused
 * through this buffer period, and cannot be unpaused.
 *
 * When the buffer period expires, it will unpause automatically, and remain permissionless forever after.
 */
contract FactoryWidePauseWindow {
    // This contract relies on timestamps - the usual caveats apply.
    // solhint-disable not-rely-on-time

    // The pause window end time is stored in 32 bits.
    uint32 private constant _MAX_TIMESTAMP = type(uint32).max;

    uint32 private immutable _pauseWindowDuration;

    // Time when the pause window for all created Pools expires.
    uint32 private immutable _poolsPauseWindowEndTime;

    /// @notice The factory deployer gave a duration that would overflow the Unix timestamp.
    error PoolPauseWindowDurationOverflow();

    constructor(uint32 pauseWindowDuration) {
        uint256 pauseWindowEndTime = block.timestamp + pauseWindowDuration;

        if (pauseWindowEndTime > _MAX_TIMESTAMP) {
            revert PoolPauseWindowDurationOverflow();
        }

        _pauseWindowDuration = pauseWindowDuration;

        // Direct cast is safe, as it was checked above.
        _poolsPauseWindowEndTime = uint32(pauseWindowEndTime);
    }

    /**
     * @notice Return the pause window duration. This is the time pools will be pausable after factory deployment.
     * @return pauseWindowDuration The duration in seconds
     */
    function getPauseWindowDuration() external view returns (uint32) {
        return _pauseWindowDuration;
    }

    /**
     * @notice Returns the original factory pauseWindowEndTime, regardless of the current time.
     * @return pauseWindowEndTime The end time as a timestamp
     */
    function getOriginalPauseWindowEndTime() external view returns (uint32) {
        return _poolsPauseWindowEndTime;
    }

    /**
     * @notice Returns the current pauseWindowEndTime that will be applied to Pools created by this factory.
     * @dev We intend for all pools deployed by this factory to have the same pause window end time (i.e., after
     * this date, all future pools will be unpausable). This function will return `_poolsPauseWindowEndTime`
     * until it passes, after which it will return 0.
     *
     * @return pauseWindowEndTime The resolved pause window end time (0 indicating it's no longer pausable)
     */
    function getNewPoolPauseWindowEndTime() public view returns (uint32) {
        // We know _poolsPauseWindowEndTime <= _MAX_TIMESTAMP (checked above).
        // Do not truncate timestamp; it should still return 0 after _MAX_TIMESTAMP.
        return (block.timestamp < _poolsPauseWindowEndTime) ? _poolsPauseWindowEndTime : 0;
    }
}

```


## ./pkg/solidity-utils/contracts/openzeppelin/StorageSlotExtension.sol

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

// pkg/solidity-utils/contracts/openzeppelin/StorageSlotExtension.sol

/**
 * @notice Library for reading and writing primitive types to specific storage slots. Based on OpenZeppelin; just adding support for int256.
 * @dev TIP: Consider using this library along with {SlotDerivation}.
 */
library StorageSlotExtension {
    struct Int256Slot {
        int256 value;
    }

    /// @dev Returns an `Int256Slot` with member `value` located at `slot`.
    function getInt256Slot(bytes32 slot) internal pure returns (Int256Slot storage r) {
        /// @solidity memory-safe-assembly
        assembly {
            r.slot := slot
        }
    }

    /// @dev Custom type that represents a slot holding an address.
    type AddressSlotType is bytes32;

    /// @dev Cast an arbitrary slot to a AddressSlotType.
    function asAddress(bytes32 slot) internal pure returns (AddressSlotType) {
        return AddressSlotType.wrap(slot);
    }

    /// @dev Custom type that represents a slot holding a boolean.
    type BooleanSlotType is bytes32;

    /// @dev Cast an arbitrary slot to a BooleanSlotType.
    function asBoolean(bytes32 slot) internal pure returns (BooleanSlotType) {
        return BooleanSlotType.wrap(slot);
    }

    /// @dev Custom type that represents a slot holding a bytes32.
    type Bytes32SlotType is bytes32;

    /// @dev Cast an arbitrary slot to a Bytes32SlotType.
    function asBytes32(bytes32 slot) internal pure returns (Bytes32SlotType) {
        return Bytes32SlotType.wrap(slot);
    }

    /// @dev Custom type that represents a slot holding a uint256.
    type Uint256SlotType is bytes32;

    /// @dev Cast an arbitrary slot to a Uint256SlotType.
    function asUint256(bytes32 slot) internal pure returns (Uint256SlotType) {
        return Uint256SlotType.wrap(slot);
    }

    /// @dev Custom type that represents a slot holding an int256.
    type Int256SlotType is bytes32;

    /// @dev Cast an arbitrary slot to an Int256SlotType.
    function asInt256(bytes32 slot) internal pure returns (Int256SlotType) {
        return Int256SlotType.wrap(slot);
    }

    /// @dev Load the value held at location `slot` in transient storage.
    function tload(AddressSlotType slot) internal view returns (address value) {
        /// @solidity memory-safe-assembly
        assembly {
            value := tload(slot)
        }
    }

    /// @dev Store `value` at location `slot` in transient storage.
    function tstore(AddressSlotType slot, address value) internal {
        /// @solidity memory-safe-assembly
        assembly {
            tstore(slot, value)
        }
    }

    /// @dev Load the value held at location `slot` in transient storage.
    function tload(BooleanSlotType slot) internal view returns (bool value) {
        /// @solidity memory-safe-assembly
        assembly {
            value := tload(slot)
        }
    }

    /// @dev Store `value` at location `slot` in transient storage.
    function tstore(BooleanSlotType slot, bool value) internal {
        /// @solidity memory-safe-assembly
        assembly {
            tstore(slot, value)
        }
    }

    /// @dev Load the value held at location `slot` in transient storage.
    function tload(Bytes32SlotType slot) internal view returns (bytes32 value) {
        /// @solidity memory-safe-assembly
        assembly {
            value := tload(slot)
        }
    }

    /// @dev Store `value` at location `slot` in transient storage.
    function tstore(Bytes32SlotType slot, bytes32 value) internal {
        /// @solidity memory-safe-assembly
        assembly {
            tstore(slot, value)
        }
    }

    /// @dev Load the value held at location `slot` in transient storage.
    function tload(Uint256SlotType slot) internal view returns (uint256 value) {
        /// @solidity memory-safe-assembly
        assembly {
            value := tload(slot)
        }
    }

    /// @dev Store `value` at location `slot` in transient storage.
    function tstore(Uint256SlotType slot, uint256 value) internal {
        /// @solidity memory-safe-assembly
        assembly {
            tstore(slot, value)
        }
    }

    /// @dev Load the value held at location `slot` in transient storage.
    function tload(Int256SlotType slot) internal view returns (int256 value) {
        /// @solidity memory-safe-assembly
        assembly {
            value := tload(slot)
        }
    }

    /// @dev Store `value` at location `slot` in transient storage.
    function tstore(Int256SlotType slot, int256 value) internal {
        /// @solidity memory-safe-assembly
        assembly {
            tstore(slot, value)
        }
    }
}

```


## ./pkg/solidity-utils/contracts/openzeppelin/Arrays.sol

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

// pkg/solidity-utils/contracts/openzeppelin/Arrays.sol

// OpenZeppelin Contracts (last updated v5.1.0) (utils/Arrays.sol)
// This file was procedurally generated from scripts/generate/templates/Arrays.js.

// NOTE: This file copied only the sort and necessary helper functions from Arrays.sol (OpenZeppelin Contracts v5.1.0)

/**
 * @dev Standard math utilities missing in the Solidity language.
 */
library Math {
    /**
     * @dev Returns the average of two numbers. The result is rounded towards
     * zero.
     */
    function average(uint256 a, uint256 b) internal pure returns (uint256) {
        // (a + b) / 2 can overflow.
        return (a & b) + (a ^ b) / 2;
    }
}

/**
 * @dev Provides a set of functions to compare values.
 *
 * _Available since v5.1._
 */
library Comparators {
    function lt(uint256 a, uint256 b) internal pure returns (bool) {
        return a < b;
    }

    function gt(uint256 a, uint256 b) internal pure returns (bool) {
        return a > b;
    }
}

/**
 * @dev Collection of functions related to array types.
 */
library Arrays {
    /**
     * @dev Sort an array of uint256 (in memory) following the provided comparator function.
     *
     * This function does the sorting "in place", meaning that it overrides the input. The object is returned for
     * convenience, but that returned value can be discarded safely if the caller has a memory pointer to the array.
     *
     * NOTE: this function's cost is `O(n · log(n))` in average and `O(n²)` in the worst case, with n the length of the
     * array. Using it in view functions that are executed through `eth_call` is safe, but one should be very careful
     * when executing this as part of a transaction. If the array being sorted is too large, the sort operation may
     * consume more gas than is available in a block, leading to potential DoS.
     *
     * IMPORTANT: Consider memory side-effects when using custom comparator functions that access memory in an
     * unsafe way.
     */
    function sort(
        uint256[] memory array,
        function(uint256, uint256) pure returns (bool) comp
    ) internal pure returns (uint256[] memory) {
        _quickSort(_begin(array), _end(array), comp);
        return array;
    }

    /**
     * @dev Variant of {sort} that sorts an array of uint256 in increasing order.
     */
    function sort(uint256[] memory array) internal pure returns (uint256[] memory) {
        sort(array, Comparators.lt);
        return array;
    }

    /**
     * @dev Sort an array of address (in memory) following the provided comparator function.
     *
     * This function does the sorting "in place", meaning that it overrides the input. The object is returned for
     * convenience, but that returned value can be discarded safely if the caller has a memory pointer to the array.
     *
     * NOTE: this function's cost is `O(n · log(n))` in average and `O(n²)` in the worst case, with n the length of the
     * array. Using it in view functions that are executed through `eth_call` is safe, but one should be very careful
     * when executing this as part of a transaction. If the array being sorted is too large, the sort operation may
     * consume more gas than is available in a block, leading to potential DoS.
     *
     * IMPORTANT: Consider memory side-effects when using custom comparator functions that access memory in an
     * unsafe way.
     */
    function sort(
        address[] memory array,
        function(address, address) pure returns (bool) comp
    ) internal pure returns (address[] memory) {
        sort(_castToUint256Array(array), _castToUint256Comp(comp));
        return array;
    }

    /**
     * @dev Variant of {sort} that sorts an array of address in increasing order.
     */
    function sort(address[] memory array) internal pure returns (address[] memory) {
        sort(_castToUint256Array(array), Comparators.lt);
        return array;
    }

    /**
     * @dev Sort an array of bytes32 (in memory) following the provided comparator function.
     *
     * This function does the sorting "in place", meaning that it overrides the input. The object is returned for
     * convenience, but that returned value can be discarded safely if the caller has a memory pointer to the array.
     *
     * NOTE: this function's cost is `O(n · log(n))` in average and `O(n²)` in the worst case, with n the length of the
     * array. Using it in view functions that are executed through `eth_call` is safe, but one should be very careful
     * when executing this as part of a transaction. If the array being sorted is too large, the sort operation may
     * consume more gas than is available in a block, leading to potential DoS.
     *
     * IMPORTANT: Consider memory side-effects when using custom comparator functions that access memory in an
     * unsafe way.
     */
    function sort(
        bytes32[] memory array,
        function(bytes32, bytes32) pure returns (bool) comp
    ) internal pure returns (bytes32[] memory) {
        sort(_castToUint256Array(array), _castToUint256Comp(comp));
        return array;
    }

    /**
     * @dev Variant of {sort} that sorts an array of bytes32 in increasing order.
     */
    function sort(bytes32[] memory array) internal pure returns (bytes32[] memory) {
        sort(_castToUint256Array(array), Comparators.lt);
        return array;
    }

    /**
     * @dev Performs a quick sort of a segment of memory. The segment sorted starts at `begin` (inclusive), and stops
     * at end (exclusive). Sorting follows the `comp` comparator.
     *
     * Invariant: `begin <= end`. This is the case when initially called by {sort} and is preserved in sub-calls.
     *
     * IMPORTANT: Memory locations between `begin` and `end` are not validated/zeroed. This function should
     * be used only if the limits are within a memory array.
     */
    function _quickSort(uint256 begin, uint256 end, function(uint256, uint256) pure returns (bool) comp) private pure {
        unchecked {
            if (end - begin < 0x40) return;

            // Use first element as pivot
            uint256 pivot = _mload(begin);
            // LiquidityPosition where the pivot should be at the end of the loop
            uint256 pos = begin;

            for (uint256 it = begin + 0x20; it < end; it += 0x20) {
                if (comp(_mload(it), pivot)) {
                    // If the value stored at the iterator's position comes before the pivot, we increment the
                    // position of the pivot and move the value there.
                    pos += 0x20;
                    _swap(pos, it);
                }
            }

            _swap(begin, pos); // Swap pivot into place
            _quickSort(begin, pos, comp); // Sort the left side of the pivot
            _quickSort(pos + 0x20, end, comp); // Sort the right side of the pivot
        }
    }

    // solhint-disable no-inline-assembly

    /**
     * @dev Pointer to the memory location of the first element of `array`.
     */
    function _begin(uint256[] memory array) private pure returns (uint256 ptr) {
        assembly ("memory-safe") {
            ptr := add(array, 0x20)
        }
    }

    /**
     * @dev Pointer to the memory location of the first memory word (32bytes) after `array`. This is the memory word
     * that comes just after the last element of the array.
     */
    function _end(uint256[] memory array) private pure returns (uint256 ptr) {
        unchecked {
            return _begin(array) + array.length * 0x20;
        }
    }

    /**
     * @dev Load memory word (as a uint256) at location `ptr`.
     */
    function _mload(uint256 ptr) private pure returns (uint256 value) {
        assembly {
            value := mload(ptr)
        }
    }

    /**
     * @dev Swaps the elements memory location `ptr1` and `ptr2`.
     */
    function _swap(uint256 ptr1, uint256 ptr2) private pure {
        assembly {
            let value1 := mload(ptr1)
            let value2 := mload(ptr2)
            mstore(ptr1, value2)
            mstore(ptr2, value1)
        }
    }

    /// @dev Helper: low level cast address memory array to uint256 memory array
    function _castToUint256Array(address[] memory input) private pure returns (uint256[] memory output) {
        assembly {
            output := input
        }
    }

    /// @dev Helper: low level cast bytes32 memory array to uint256 memory array
    function _castToUint256Array(bytes32[] memory input) private pure returns (uint256[] memory output) {
        assembly {
            output := input
        }
    }

    /// @dev Helper: low level cast address comp function to uint256 comp function
    function _castToUint256Comp(
        function(address, address) pure returns (bool) input
    ) private pure returns (function(uint256, uint256) pure returns (bool) output) {
        assembly {
            output := input
        }
    }

    /// @dev Helper: low level cast bytes32 comp function to uint256 comp function
    function _castToUint256Comp(
        function(bytes32, bytes32) pure returns (bool) input
    ) private pure returns (function(uint256, uint256) pure returns (bool) output) {
        assembly {
            output := input
        }
    }
}

```


## ./pkg/solidity-utils/contracts/openzeppelin/EnumerableSet.sol

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

// pkg/solidity-utils/contracts/openzeppelin/EnumerableSet.sol

/**
 * @notice  Library for managing sets of primitive types.
 * @dev See https://en.wikipedia.org/wiki/Set_(abstract_dataType)[sets] of primitive types.
 *
 * Based on the EnumerableSet library from OpenZeppelin Contracts, altered to remove the base private functions that
 * work on bytes32, replacing them with a native implementation for address values, to reduce bytecode size and
 * runtime costs.
 *
 * The `uncheckedAt` function was also added, which allows for more gas efficient data reads in some scenarios.
 *
 * Sets have the following properties:
 *
 * - Elements are added, removed, and checked for existence in constant time (O(1)).
 * - Elements are enumerated in O(n). No guarantees are made on the ordering.
 *
 * ```
 * contract Example {
 *     // Add the library methods
 *     using EnumerableSet for EnumerableSet.AddressSet;
 *
 *     // Declare a set state variable
 *     EnumerableSet.AddressSet private mySet;
 * }
 * ```
 */
library EnumerableSet {
    // The original OpenZeppelin implementation uses a generic Set type with bytes32 values: this was replaced with
    // AddressSet, which uses address keys natively, resulting in more dense bytecode.

    // solhint-disable func-name-mixedcase

    struct AddressSet {
        // Storage of set values.
        address[] _values;
        // LiquidityPosition of the value in the `values` array, plus 1 because index 0
        // means a value is not in the set.
        mapping(address addressKey => uint256 indexValue) Indexes;
    }

    /// @notice An index is beyond the current bounds of the set.
    error IndexOutOfBounds();

    /// @notice An element that is not present in the set.
    error ElementNotFound();

    /**
     * @dev Add a value to a set. O(1).
     *
     * Returns true if the value was added to the set, if it was not already present.
     */
    function add(AddressSet storage set, address value) internal returns (bool) {
        if (!contains(set, value)) {
            set._values.push(value);
            // The value is stored at length-1, but we add 1 to all indexes
            // and use 0 as a sentinel value.
            set.Indexes[value] = set._values.length;
            return true;
        } else {
            return false;
        }
    }

    /**
     * @dev Removes a value from a set. O(1).
     * Returns true if the value was removed from the set; i.e., if it was present.
     */
    function remove(AddressSet storage set, address value) internal returns (bool) {
        // We read and store the value's index to prevent multiple reads from the same storage slot.
        uint256 valueIndex = set.Indexes[value];

        if (valueIndex != 0) {
            // Equivalent to contains(set, value)
            // To delete an element from the _values array in O(1), we swap the element to delete with the last one in
            // the array, and then remove the last element (sometimes called as 'swap and pop').
            // This modifies the order of the array, as noted in {at}.
            uint256 toDeleteIndex;
            uint256 lastIndex;

            unchecked {
                toDeleteIndex = valueIndex - 1;
                lastIndex = set._values.length - 1;
            }

            // The swap is only necessary if we're not removing the last element.
            if (toDeleteIndex != lastIndex) {
                address lastValue = set._values[lastIndex];

                // Move the last entry to the index of the entry to delete.
                set._values[toDeleteIndex] = lastValue;
                // Update the index for the moved value
                set.Indexes[lastValue] = valueIndex; // = toDeleteIndex + 1; all indices are 1-based.
            }

            // Delete the slot where the moved value was stored.
            set._values.pop();

            // Delete the index for the deleted slot.
            delete set.Indexes[value];

            return true;
        } else {
            return false;
        }
    }

    /// @dev Returns true if the value is in the set. O(1).
    function contains(AddressSet storage set, address value) internal view returns (bool) {
        return set.Indexes[value] != 0;
    }

    /// @dev Returns the number of values on the set. O(1).
    function length(AddressSet storage set) internal view returns (uint256) {
        return set._values.length;
    }

    /**
     * @dev Returns the value stored at position `index` in the set. O(1).
     *
     * Note that there are no guarantees on the ordering of values inside the
     * array, and it may change when more values are added or removed.
     *
     * Requirements:
     *
     * - `index` must be strictly less than {length}.
     */
    function at(AddressSet storage set, uint256 index) internal view returns (address) {
        if (index >= set._values.length) {
            revert IndexOutOfBounds();
        }

        return uncheckedAt(set, index);
    }

    /**
     * @dev Same as {at}, except this doesn't revert if `index` it outside of the set (i.e. if it is equal or larger
     * than {length}). O(1).
     *
     * This function performs one less storage read than {at}, but should only be used when `index` is known to be
     * within bounds.
     */
    function uncheckedAt(AddressSet storage set, uint256 index) internal view returns (address) {
        return set._values[index];
    }

    /// @dev Return the index of an element in the set, or revert if not found.
    function indexOf(AddressSet storage set, address value) internal view returns (uint256) {
        uint256 rawIndex = set.Indexes[value];

        if (rawIndex == 0) {
            revert ElementNotFound();
        }

        unchecked {
            return rawIndex - 1;
        }
    }

    /**
     * @dev Same as {indexOf}, except this doesn't revert if the element isn't present in the set.
     * In this case, it returns 0.
     *
     * This function performs one less storage read than {indexOf}, but should only be used when `index` is known to be
     * within bounds.
     */
    function uncheckedIndexOf(AddressSet storage set, address value) internal view returns (uint256) {
        uint256 rawIndex = set.Indexes[value];

        unchecked {
            return rawIndex == 0 ? 0 : rawIndex - 1;
        }
    }
}

```


## ./pkg/solidity-utils/contracts/openzeppelin/SlotDerivation.sol

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

// pkg/solidity-utils/contracts/openzeppelin/SlotDerivation.sol

// This file was procedurally generated from scripts/generate/templates/SlotDerivation.js.

// Taken from https://raw.githubusercontent.com/Amxx/openzeppelin-contracts/ce497cb05ca05bb9aa2b86ec1d99e6454e7ab2e9/contracts/utils/SlotDerivation.sol

/**
 * @notice Library for computing storage (and transient storage) locations from namespaces and deriving slots
 * corresponding to standard patterns.
 * @dev The derivation method for array and mapping matches the storage layout used by the solidity language/compiler.
 *
 * See https://docs.soliditylang.org/en/v0.8.20/internals/layout_in_storage.html#mappings-and-dynamic-arrays[Solidity docs for mappings and dynamic arrays.].
 *
 * Example usage:
 * ```solidity
 * contract Example {
 *     // Add the library methods
 *     using StorageSlot for bytes32;
 *     using SlotDerivation for bytes32;
 *
 *     // Declare a namespace
 *     string private constant _NAMESPACE = "<namespace>" // eg. OpenZeppelin.Slot
 *
 *     function setValueInNamespace(uint256 key, address newValue) internal {
 *         _NAMESPACE.erc7201Slot().deriveMapping(key).getAddressSlot().value = newValue;
 *     }
 *
 *     function getValueInNamespace(uint256 key) internal view returns (address) {
 *         return _NAMESPACE.erc7201Slot().deriveMapping(key).getAddressSlot().value;
 *     }
 * }
 * ```
 *
 * TIP: Consider using this library along with {StorageSlot}.
 *
 * NOTE: This library provides a way to manipulate storage locations in a non-standard way. Tooling for checking
 * upgrade safety will ignore the slots accessed through this library.
 */
library SlotDerivation {
    /// @dev Derive an ERC-7201 slot from a string (namespace).
    function erc7201Slot(string memory namespace) internal pure returns (bytes32 slot) {
        /// @solidity memory-safe-assembly
        assembly {
            mstore(0x00, sub(keccak256(add(namespace, 0x20), mload(namespace)), 1))
            slot := and(keccak256(0x00, 0x20), not(0xff))
        }
    }

    /// @dev Add an offset to a slot to get the n-th element of a structure or an array.
    function offset(bytes32 slot, uint256 pos) internal pure returns (bytes32 result) {
        unchecked {
            return bytes32(uint256(slot) + pos);
        }
    }

    /// @dev Derive the location of the first element in an array from the slot where the length is stored.
    function deriveArray(bytes32 slot) internal pure returns (bytes32 result) {
        /// @solidity memory-safe-assembly
        assembly {
            mstore(0x00, slot)
            result := keccak256(0x00, 0x20)
        }
    }

    /// @dev Derive the location of a mapping element from the key.
    function deriveMapping(bytes32 slot, address key) internal pure returns (bytes32 result) {
        /// @solidity memory-safe-assembly
        assembly {
            mstore(0x00, key)
            mstore(0x20, slot)
            result := keccak256(0x00, 0x40)
        }
    }

    /// @dev Derive the location of a mapping element from the key.
    function deriveMapping(bytes32 slot, bool key) internal pure returns (bytes32 result) {
        /// @solidity memory-safe-assembly
        assembly {
            mstore(0x00, key)
            mstore(0x20, slot)
            result := keccak256(0x00, 0x40)
        }
    }

    /// @dev Derive the location of a mapping element from the key.
    function deriveMapping(bytes32 slot, bytes32 key) internal pure returns (bytes32 result) {
        /// @solidity memory-safe-assembly
        assembly {
            mstore(0x00, key)
            mstore(0x20, slot)
            result := keccak256(0x00, 0x40)
        }
    }

    /// @dev Derive the location of a mapping element from the key.
    function deriveMapping(bytes32 slot, uint256 key) internal pure returns (bytes32 result) {
        /// @solidity memory-safe-assembly
        assembly {
            mstore(0x00, key)
            mstore(0x20, slot)
            result := keccak256(0x00, 0x40)
        }
    }

    /// @dev Derive the location of a mapping element from the key.
    function deriveMapping(bytes32 slot, int256 key) internal pure returns (bytes32 result) {
        /// @solidity memory-safe-assembly
        assembly {
            mstore(0x00, key)
            mstore(0x20, slot)
            result := keccak256(0x00, 0x40)
        }
    }

    /// @dev Derive the location of a mapping element from the key.
    function deriveMapping(bytes32 slot, string memory key) internal pure returns (bytes32 result) {
        /// @solidity memory-safe-assembly
        assembly {
            let length := mload(key)
            let begin := add(key, 0x20)
            let end := add(begin, length)
            let cache := mload(end)
            mstore(end, slot)
            result := keccak256(begin, add(length, 0x20))
            mstore(end, cache)
        }
    }

    /// @dev Derive the location of a mapping element from the key.
    function deriveMapping(bytes32 slot, bytes memory key) internal pure returns (bytes32 result) {
        /// @solidity memory-safe-assembly
        assembly {
            let length := mload(key)
            let begin := add(key, 0x20)
            let end := add(begin, length)
            let cache := mload(end)
            mstore(end, slot)
            result := keccak256(begin, add(length, 0x20))
            mstore(end, cache)
        }
    }
}

```


## ./pkg/solidity-utils/contracts/openzeppelin/ReentrancyGuardTransient.sol

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

// pkg/solidity-utils/contracts/openzeppelin/StorageSlotExtension.sol

/**
 * @notice Library for reading and writing primitive types to specific storage slots. Based on OpenZeppelin; just adding support for int256.
 * @dev TIP: Consider using this library along with {SlotDerivation}.
 */
library StorageSlotExtension {
    struct Int256Slot {
        int256 value;
    }

    /// @dev Returns an `Int256Slot` with member `value` located at `slot`.
    function getInt256Slot(bytes32 slot) internal pure returns (Int256Slot storage r) {
        /// @solidity memory-safe-assembly
        assembly {
            r.slot := slot
        }
    }

    /// @dev Custom type that represents a slot holding an address.
    type AddressSlotType is bytes32;

    /// @dev Cast an arbitrary slot to a AddressSlotType.
    function asAddress(bytes32 slot) internal pure returns (AddressSlotType) {
        return AddressSlotType.wrap(slot);
    }

    /// @dev Custom type that represents a slot holding a boolean.
    type BooleanSlotType is bytes32;

    /// @dev Cast an arbitrary slot to a BooleanSlotType.
    function asBoolean(bytes32 slot) internal pure returns (BooleanSlotType) {
        return BooleanSlotType.wrap(slot);
    }

    /// @dev Custom type that represents a slot holding a bytes32.
    type Bytes32SlotType is bytes32;

    /// @dev Cast an arbitrary slot to a Bytes32SlotType.
    function asBytes32(bytes32 slot) internal pure returns (Bytes32SlotType) {
        return Bytes32SlotType.wrap(slot);
    }

    /// @dev Custom type that represents a slot holding a uint256.
    type Uint256SlotType is bytes32;

    /// @dev Cast an arbitrary slot to a Uint256SlotType.
    function asUint256(bytes32 slot) internal pure returns (Uint256SlotType) {
        return Uint256SlotType.wrap(slot);
    }

    /// @dev Custom type that represents a slot holding an int256.
    type Int256SlotType is bytes32;

    /// @dev Cast an arbitrary slot to an Int256SlotType.
    function asInt256(bytes32 slot) internal pure returns (Int256SlotType) {
        return Int256SlotType.wrap(slot);
    }

    /// @dev Load the value held at location `slot` in transient storage.
    function tload(AddressSlotType slot) internal view returns (address value) {
        /// @solidity memory-safe-assembly
        assembly {
            value := tload(slot)
        }
    }

    /// @dev Store `value` at location `slot` in transient storage.
    function tstore(AddressSlotType slot, address value) internal {
        /// @solidity memory-safe-assembly
        assembly {
            tstore(slot, value)
        }
    }

    /// @dev Load the value held at location `slot` in transient storage.
    function tload(BooleanSlotType slot) internal view returns (bool value) {
        /// @solidity memory-safe-assembly
        assembly {
            value := tload(slot)
        }
    }

    /// @dev Store `value` at location `slot` in transient storage.
    function tstore(BooleanSlotType slot, bool value) internal {
        /// @solidity memory-safe-assembly
        assembly {
            tstore(slot, value)
        }
    }

    /// @dev Load the value held at location `slot` in transient storage.
    function tload(Bytes32SlotType slot) internal view returns (bytes32 value) {
        /// @solidity memory-safe-assembly
        assembly {
            value := tload(slot)
        }
    }

    /// @dev Store `value` at location `slot` in transient storage.
    function tstore(Bytes32SlotType slot, bytes32 value) internal {
        /// @solidity memory-safe-assembly
        assembly {
            tstore(slot, value)
        }
    }

    /// @dev Load the value held at location `slot` in transient storage.
    function tload(Uint256SlotType slot) internal view returns (uint256 value) {
        /// @solidity memory-safe-assembly
        assembly {
            value := tload(slot)
        }
    }

    /// @dev Store `value` at location `slot` in transient storage.
    function tstore(Uint256SlotType slot, uint256 value) internal {
        /// @solidity memory-safe-assembly
        assembly {
            tstore(slot, value)
        }
    }

    /// @dev Load the value held at location `slot` in transient storage.
    function tload(Int256SlotType slot) internal view returns (int256 value) {
        /// @solidity memory-safe-assembly
        assembly {
            value := tload(slot)
        }
    }

    /// @dev Store `value` at location `slot` in transient storage.
    function tstore(Int256SlotType slot, int256 value) internal {
        /// @solidity memory-safe-assembly
        assembly {
            tstore(slot, value)
        }
    }
}

// pkg/solidity-utils/contracts/openzeppelin/ReentrancyGuardTransient.sol

/**
 * @notice Variant of {ReentrancyGuard} that uses transient storage.
 * @dev NOTE: This variant only works on networks where EIP-1153 is available.
 */
abstract contract ReentrancyGuardTransient {
    using StorageSlotExtension for *;

    // keccak256(abi.encode(uint256(keccak256("openzeppelin.storage.ReentrancyGuard")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant _REENTRANCY_GUARD_STORAGE =
        0x9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f00;

    /// @notice Unauthorized reentrant call.
    error ReentrancyGuardReentrantCall();

    /**
     * @dev Prevents a contract from calling itself, directly or indirectly.
     * Calling a `nonReentrant` function from another `nonReentrant`
     * function is not supported. It is possible to prevent this from happening
     * by making the `nonReentrant` function external, and making it call a
     * `private` function that does the actual work.
     */
    modifier nonReentrant() {
        _nonReentrantBefore();
        _;
        _nonReentrantAfter();
    }

    function _nonReentrantBefore() private {
        // On the first call to nonReentrant, Status will be NOT_ENTERED.
        if (_reentrancyGuardEntered()) {
            revert ReentrancyGuardReentrantCall();
        }

        // Any calls to nonReentrant after this point will fail.
        _REENTRANCY_GUARD_STORAGE.asBoolean().tstore(true);
    }

    function _nonReentrantAfter() private {
        _REENTRANCY_GUARD_STORAGE.asBoolean().tstore(false);
    }

    /**
     * @dev Returns true if the reentrancy guard is currently set to "entered", which indicates there is a
     * `nonReentrant` function in the call stack.
     */
    function _reentrancyGuardEntered() internal view returns (bool) {
        return _REENTRANCY_GUARD_STORAGE.asBoolean().tload();
    }
}

```


## ./pkg/standalone-utils/contracts/utils/HyperTokenInfoPrecompile.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/standalone-utils/contracts/utils/HyperTokenInfoPrecompile.sol

/**
 * @notice Library to interact with the Hyperliquid token info precompile.
 * @dev The precompile is a special type of code, executed in the Hypercore's node. For more information, see
 * https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm/interacting-with-hypercore .
 */
library HyperTokenInfoPrecompile {
    // The following structure is defined by the token info precompile.
    struct HyperTokenInfo {
        string name;
        uint64[] spots;
        uint64 deployerTradingFeeShare;
        address deployer;
        address evmContract;
        uint8 szDecimals;
        uint8 weiDecimals;
        int8 evmExtraWeiDecimals;
    }

    address public constant TOKEN_INFO_PRECOMPILE_ADDRESS = 0x000000000000000000000000000000000000080C;

    /// @notice The precompile had an error while fetching the token info.
    error TokenInfoPrecompileFailed();

    function szDecimals(uint32 tokenIndex) internal view returns (uint8) {
        (bool success, bytes memory out) = TOKEN_INFO_PRECOMPILE_ADDRESS.staticcall(abi.encode(tokenIndex));
        if (success == false) {
            revert TokenInfoPrecompileFailed();
        }
        HyperTokenInfo memory tokenInfo = abi.decode(out, (HyperTokenInfo));
        return tokenInfo.szDecimals;
    }
}

```


## ./pkg/standalone-utils/contracts/utils/HyperSpotPricePrecompile.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/standalone-utils/contracts/utils/HyperSpotPricePrecompile.sol

/**
 * @notice Library to interact with the Hyperliquid spot price precompile.
 * @dev The precompile is a special type of code, executed in the Hypercore's node. For more information, see
 * https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm/interacting-with-hypercore .
 */
library HyperSpotPricePrecompile {
    address public constant SPOT_PRICE_PRECOMPILE_ADDRESS = 0x0000000000000000000000000000000000000808;

    /// @notice The precompile had an error while fetching the spot price.
    error SpotPricePrecompileFailed();

    /// @notice The spot price is zero.
    error SpotPriceIsZero();

    function spotPrice(uint32 pairIndex) internal view returns (uint256) {
        (bool success, bytes memory spotPriceBytes) = SPOT_PRICE_PRECOMPILE_ADDRESS.staticcall(abi.encode(pairIndex));
        if (success == false) {
            revert SpotPricePrecompileFailed();
        }
        uint256 price = abi.decode(spotPriceBytes, (uint256));
        if (price == 0) {
            revert SpotPriceIsZero();
        }
        return price;
    }
}

```


## ./pkg/pool-gyro/contracts/lib/SignedFixedPoint.sol

```solidity
// SPDX-License-Identifier: LicenseRef-Gyro-1.0
pragma solidity ^0.8.24;

// pkg/pool-gyro/contracts/lib/SignedFixedPoint.sol

// for information on licensing please see the README in the GitHub repository
// <https://github.com/gyrostable/concentrated-lps>.

/* solhint-disable private-vars-leading-underscore */

/**
 * @notice Signed fixed point operations based on Balancer's FixedPoint library.
 * @dev The `{mul,div}{UpMag,DownMag}()` functions do *not* round up or down, respectively, in a signed fashion (like
 * ceil and floor operations), but *in absolute value* (or *magnitude*), i.e., towards 0. This is useful in some
 * applications.
 */
library SignedFixedPoint {
    error AddOverflow();
    error SubOverflow();
    error MulOverflow();
    error ZeroDivision();
    error DivInterval();

    int256 internal constant ONE = 1e18; // 18 decimal places
    // Setting extra precision at 38 decimals, which is the most we can get without overflowing on normal
    // multiplication. This allows 20 extra digits to absorb error when multiplying by large numbers.
    int256 internal constant ONE_XP = 1e38; // 38 decimal places

    function add(int256 a, int256 b) internal pure returns (int256) {
        // Fixed Point addition is the same as regular checked addition

        int256 c = a + b;
        if (!(b >= 0 ? c >= a : c < a)) revert AddOverflow();
        return c;
    }

    function addMag(int256 a, int256 b) internal pure returns (int256 c) {
        // add b in the same signed direction as a, i.e. increase the magnitude of a by b
        c = a > 0 ? add(a, b) : sub(a, b);
    }

    function sub(int256 a, int256 b) internal pure returns (int256) {
        // Fixed Point subtraction is the same as regular checked subtraction

        int256 c = a - b;
        if (!(b <= 0 ? c >= a : c < a)) revert SubOverflow();
        return c;
    }

    /// @dev This rounds towards 0, i.e., down *in absolute value*!
    function mulDownMag(int256 a, int256 b) internal pure returns (int256) {
        int256 product = a * b;
        if (!(a == 0 || product / a == b)) revert MulOverflow();

        return product / ONE;
    }

    /**
     * @dev This implements mulDownMag without checking for over/under-flows, which saves significantly on gas if these
     * aren't needed
     */
    function mulDownMagU(int256 a, int256 b) internal pure returns (int256) {
        return (a * b) / ONE;
    }

    /// @dev This rounds away from 0, i.e., up *in absolute value*!
    function mulUpMag(int256 a, int256 b) internal pure returns (int256) {
        int256 product = a * b;
        if (!(a == 0 || product / a == b)) revert MulOverflow();

        // If product > 0, the result should be ceil(p/ONE) = floor((p-1)/ONE) + 1, where floor() is implicit. If
        // product < 0, the result should be floor(p/ONE) = ceil((p+1)/ONE) - 1, where ceil() is implicit.
        // Addition for signed numbers: Case selection so we round away from 0, not always up.
        if (product > 0) return ((product - 1) / ONE) + 1;
        else if (product < 0) return ((product + 1) / ONE) - 1;
        // product == 0
        return 0;
    }

    /**
     * @dev this implements mulUpMag without checking for over/under-flows, which saves significantly on gas if these
     * aren't needed
     */
    function mulUpMagU(int256 a, int256 b) internal pure returns (int256) {
        int256 product = a * b;

        // If product > 0, the result should be ceil(p/ONE) = floor((p-1)/ONE) + 1, where floor() is implicit. If
        // product < 0, the result should be floor(p/ONE) = ceil((p+1)/ONE) - 1, where ceil() is implicit.
        // Addition for signed numbers: Case selection so we round away from 0, not always up.
        if (product > 0) return ((product - 1) / ONE) + 1;
        else if (product < 0) return ((product + 1) / ONE) - 1;
        // product == 0
        return 0;
    }

    /// @dev Rounds towards 0, i.e., down in absolute value.
    function divDownMag(int256 a, int256 b) internal pure returns (int256) {
        if (b == 0) revert ZeroDivision();

        if (a == 0) {
            return 0;
        }

        int256 aInflated = a * ONE;
        if (aInflated / a != ONE) revert DivInterval();

        return aInflated / b;
    }

    /**
     * @dev this implements divDownMag without checking for over/under-flows, which saves significantly on gas if these
     * aren't needed
     */
    function divDownMagU(int256 a, int256 b) internal pure returns (int256) {
        if (b == 0) revert ZeroDivision();
        return (a * ONE) / b;
    }

    /// @dev Rounds away from 0, i.e., up in absolute value.
    function divUpMag(int256 a, int256 b) internal pure returns (int256) {
        if (b == 0) revert ZeroDivision();

        if (a == 0) {
            return 0;
        }

        if (b < 0) {
            // Required so the below is correct.
            b = -b;
            a = -a;
        }

        int256 aInflated = a * ONE;
        if (aInflated / a != ONE) revert DivInterval();

        if (aInflated > 0) return ((aInflated - 1) / b) + 1;
        return ((aInflated + 1) / b) - 1;
    }

    /**
     * @dev this implements divUpMag without checking for over/under-flows, which saves significantly on gas if these
     * aren't needed
     */
    function divUpMagU(int256 a, int256 b) internal pure returns (int256) {
        if (b == 0) revert ZeroDivision();

        if (a == 0) {
            return 0;
        }

        // SOMEDAY check if we can shave off some gas by logically refactoring this vs the below case distinction
        // into one (on a * b or so).
        if (b < 0) {
            // Ensure b > 0 so the below is correct.
            b = -b;
            a = -a;
        }

        if (a > 0) return ((a * ONE - 1) / b) + 1;
        return ((a * ONE + 1) / b) - 1;
    }

    /**
     * @notice Multiplies two extra precision numbers (with 38 decimals).
     * @dev Rounds down in magnitude but this shouldn't matter. Multiplication can overflow if a,b are > 2 in
     * magnitude.
     */
    function mulXp(int256 a, int256 b) internal pure returns (int256) {
        int256 product = a * b;
        if (!(a == 0 || product / a == b)) revert MulOverflow();

        return product / ONE_XP;
    }

    /**
     * @notice Multiplies two extra precision numbers (with 38 decimals).
     * @dev Rounds down in magnitude but this shouldn't matter. Multiplication can overflow if a,b are > 2 in
     * magnitude. This implements mulXp without checking for over/under-flows, which saves significantly on gas if
     * these aren't needed.
     */
    function mulXpU(int256 a, int256 b) internal pure returns (int256) {
        return (a * b) / ONE_XP;
    }

    /**
     * @notice @notice Divides two extra precision numbers (with 38 decimals).
     * @dev Rounds down in magnitude but this shouldn't matter. Division can overflow if a > 2 or b << 1 in magnitude.
     */
    function divXp(int256 a, int256 b) internal pure returns (int256) {
        if (b == 0) revert ZeroDivision();

        if (a == 0) {
            return 0;
        }

        int256 aInflated = a * ONE_XP;
        if (aInflated / a != ONE_XP) revert DivInterval();

        return aInflated / b;
    }

    /**
     * @notice Divides two extra precision numbers (with 38 decimals).
     * @dev Rounds down in magnitude but this shouldn't matter. Division can overflow if a > 2 or b << 1 in magnitude.
     * This implements divXp without checking for over/under-flows, which saves significantly on gas if these aren't
     * needed.
     */
    function divXpU(int256 a, int256 b) internal pure returns (int256) {
        if (b == 0) revert ZeroDivision();

        return (a * ONE_XP) / b;
    }

    /**
     * @notice Multiplies normal precision a with extra precision b (with 38 decimals).
     * @dev Rounds down in signed direction. Returns normal precision of the product.
     */
    function mulDownXpToNp(int256 a, int256 b) internal pure returns (int256) {
        int256 b1 = b / 1e19;
        int256 prod1 = a * b1;
        if (!(a == 0 || prod1 / a == b1)) revert MulOverflow();
        int256 b2 = b % 1e19;
        int256 prod2 = a * b2;
        if (!(a == 0 || prod2 / a == b2)) revert MulOverflow();
        return prod1 >= 0 && prod2 >= 0 ? (prod1 + prod2 / 1e19) / 1e19 : (prod1 + prod2 / 1e19 + 1) / 1e19 - 1;
    }

    /**
     * @notice Multiplies normal precision a with extra precision b (with 38 decimals).
     * @dev Rounds down in signed direction. Returns normal precision of the product. This implements mulDownXpToNp
     * without checking for over/under-flows, which saves significantly on gas if these aren't needed.
     */
    function mulDownXpToNpU(int256 a, int256 b) internal pure returns (int256) {
        int256 b1 = b / 1e19;
        int256 b2 = b % 1e19;
        // SOMEDAY check if we eliminate these vars and save some gas (by only checking the sign of prod1, say)
        int256 prod1 = a * b1;
        int256 prod2 = a * b2;
        return prod1 >= 0 && prod2 >= 0 ? (prod1 + prod2 / 1e19) / 1e19 : (prod1 + prod2 / 1e19 + 1) / 1e19 - 1;
    }

    /**
     * @notice Multiplies normal precision a with extra precision b (with 38 decimals).
     * @dev Rounds down in signed direction. Returns normal precision of the product.
     */
    function mulUpXpToNp(int256 a, int256 b) internal pure returns (int256) {
        int256 b1 = b / 1e19;
        int256 prod1 = a * b1;
        if (!(a == 0 || prod1 / a == b1)) revert MulOverflow();
        int256 b2 = b % 1e19;
        int256 prod2 = a * b2;
        if (!(a == 0 || prod2 / a == b2)) revert MulOverflow();
        return prod1 <= 0 && prod2 <= 0 ? (prod1 + prod2 / 1e19) / 1e19 : (prod1 + prod2 / 1e19 - 1) / 1e19 + 1;
    }

    /**
     * @notice Multiplies normal precision a with extra precision b (with 38 decimals).
     * @dev Rounds down in signed direction. Returns normal precision of the product. This implements mulUpXpToNp
     * without checking for over/under-flows, which saves significantly on gas if these aren't needed.
     */
    function mulUpXpToNpU(int256 a, int256 b) internal pure returns (int256) {
        int256 b1 = b / 1e19;
        int256 b2 = b % 1e19;
        // SOMEDAY check if we eliminate these vars and save some gas (by only checking the sign of prod1, say).
        int256 prod1 = a * b1;
        int256 prod2 = a * b2;
        return prod1 <= 0 && prod2 <= 0 ? (prod1 + prod2 / 1e19) / 1e19 : (prod1 + prod2 / 1e19 - 1) / 1e19 + 1;
    }

    /**
     * @notice Returns the complement of a value (1 - x), capped to 0 if x is larger than 1.
     * @dev Useful when computing the complement for values with some level of relative error, as it strips this
     * error and prevents intermediate negative values.
     */
    function complement(int256 x) internal pure returns (int256) {
        if (x >= ONE || x <= 0) return 0;
        return ONE - x;
    }
}

```


## ./pkg/interfaces/contracts/solidity-utils/helpers/IPoolVersion.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/solidity-utils/helpers/IPoolVersion.sol

/// @notice Simple interface to retrieve the version of pools deployed by a pool factory.
interface IPoolVersion {
    /**
     * @notice Returns a JSON representation of the deployed pool version containing name, version number and task ID.
     * @dev This is typically only useful in complex Pool deployment schemes, where multiple subsystems need to know
     * about each other. Note that this value will only be set at factory creation time.
     *
     * @return poolVersion A string representation of the pool version
     */
    function getPoolVersion() external view returns (string memory poolVersion);
}

```


## ./pkg/interfaces/contracts/solidity-utils/helpers/IAuthentication.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/solidity-utils/helpers/IAuthentication.sol

/// @notice Simple interface for permissioned calling of external functions.
interface IAuthentication {
    /// @notice The sender does not have permission to call a function.
    error SenderNotAllowed();

    /**
     * @notice Returns the action identifier associated with the external function described by `selector`.
     * @param selector The 4-byte selector of the permissioned function
     * @return actionId The computed actionId
     */
    function getActionId(bytes4 selector) external view returns (bytes32 actionId);
}

```


## ./pkg/interfaces/contracts/solidity-utils/helpers/IRateProvider.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/solidity-utils/helpers/IRateProvider.sol

/// @notice General interface for token exchange rates.
interface IRateProvider {
    /**
     * @notice An 18 decimal fixed point number representing the exchange rate of one token to another related token.
     * @dev The meaning of this rate depends on the context. Note that there may be an error associated with a token
     * rate, and the caller might require a certain rounding direction to ensure correctness. This (legacy) interface
     * does not take a rounding direction or return an error, so great care must be taken when interpreting and using
     * rates in downstream computations.
     *
     * @return rate The current token rate
     */
    function getRate() external view returns (uint256 rate);
}

```


## ./pkg/interfaces/contracts/solidity-utils/helpers/IVersion.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/solidity-utils/helpers/IVersion.sol

/// @notice Simple interface to retrieve the version of a deployed contract.
interface IVersion {
    /**
     * @notice Return arbitrary text representing the version of a contract.
     * @dev For standard Balancer contracts, returns a JSON representation of the contract version containing name,
     * version number and task ID. See real examples in the deployment repo; local tests just use plain text strings.
     *
     * @return version The version string corresponding to the current deployed contract
     */
    function version() external view returns (string memory);
}

```


## ./pkg/interfaces/contracts/oracles/IWeightedLPOracle.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/oracles/IWeightedLPOracle.sol

interface IWeightedLPOracle {
    /**
     * @notice Gets the current weights of the tokens in the pool.
     * @return weights An array of weights corresponding to each token in the pool
     */
    function getWeights() external view returns (uint256[] memory weights);
}

```


## ./pkg/interfaces/contracts/governance-scripts/IBasicAuthorizer.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/vault/IAuthorizer.sol

/// @notice Interface to the Vault's permission system.
interface IAuthorizer {
    /**
     * @notice Returns true if `account` can perform the action described by `actionId` in the contract `where`.
     * @param actionId Identifier for the action to be performed
     * @param account Account trying to perform the action
     * @param where Target contract for the action
     * @return success True if the action is permitted
     */
    function canPerform(bytes32 actionId, address account, address where) external view returns (bool success);
}

// pkg/interfaces/contracts/governance-scripts/IBasicAuthorizer.sol

interface IBasicAuthorizer is IAuthorizer {
    // solhint-disable-next-line func-name-mixedcase
    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);

    function grantRole(bytes32 role, address account) external;

    function revokeRole(bytes32 role, address account) external;

    function renounceRole(bytes32 role, address account) external;
}

```


## ./pkg/interfaces/contracts/pool-hooks/IECLPSurgeHook.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/pool-hooks/IECLPSurgeHook.sol

interface IECLPSurgeHook {
    /// @notice Thrown when an invalid imbalance slope is provided.
    error InvalidImbalanceSlope();

    /**
     * @notice The rotation angle is too small or too large for the surge hook to be used.
     * @dev The surge hook accepts angles from 30 to 60 degrees. Outside of this range, the computation of the peak
     * price cannot be approximated by sine/cosine.
     */
    error InvalidRotationAngle();

    /**
     * @notice A new `ECLPSurgeHook` contract has been registered successfully.
     * @dev If the registration fails the call will revert, so there will be no event.
     * @param pool The pool on which the hook was registered
     * @param factory The factory that registered the pool
     */
    event ECLPSurgeHookRegistered(address indexed pool, address indexed factory);

    /**
     * @notice The imbalance slope below peak has been changed for a pool in a `ECLPSurgeHook` contract.
     * @dev Note, the initial imbalance slope below peak is set on deployment, and an event is emitted.
     * @param pool The pool for which the imbalance slope below peak has been changed
     * @param newImbalanceSlopeBelowPeak The new imbalance slope below peak
     */
    event ImbalanceSlopeBelowPeakChanged(address indexed pool, uint128 newImbalanceSlopeBelowPeak);

    /**
     * @notice The imbalance slope above peak has been changed for a pool in a `ECLPSurgeHook` contract.
     * @dev Note, the initial imbalance slope above peak is set on deployment, and an event is emitted.
     * @param pool The pool for which the imbalance slope above peak has been changed
     * @param newImbalanceSlopeAbovePeak The new imbalance slope above peak
     */
    event ImbalanceSlopeAbovePeakChanged(address indexed pool, uint128 newImbalanceSlopeAbovePeak);

    /**
     * @notice Getter for the imbalance slope below peak for a pool.
     * @param pool The pool for which the imbalance slope below peak is requested
     * @return imbalanceSlopeBelowPeak The imbalance slope below peak for the pool
     * @return imbalanceSlopeAbovePeak The imbalance slope above peak for the pool
     */
    function getImbalanceSlopes(
        address pool
    ) external view returns (uint256 imbalanceSlopeBelowPeak, uint256 imbalanceSlopeAbovePeak);

    /**
     * @notice Sets the imbalance slope below peak for a pool.
     * @dev This function must be permissioned. If the pool does not have a swap fee manager role set, the imbalance
     * slope below peak can only be changed by governance. It is initially set to the default imbalance slope for this
     * hook contract.
     *
     * @param pool The pool for which the imbalance slope below peak is being set
     * @param newImbalanceSlopeBelowPeak The new imbalance slope below peak
     */
    function setImbalanceSlopeBelowPeak(address pool, uint256 newImbalanceSlopeBelowPeak) external;

    /**
     * @notice Sets the imbalance slope above peak for a pool.
     * @dev This function must be permissioned. If the pool does not have a swap fee manager role set, the imbalance
     * slope above peak can only be changed by governance. It is initially set to the default imbalance slope for this
     * hook contract.
     *
     * @param pool The pool for which the imbalance slope above peak is being set
     * @param newImbalanceSlopeAbovePeak The new imbalance slope above peak
     */
    function setImbalanceSlopeAbovePeak(address pool, uint256 newImbalanceSlopeAbovePeak) external;
}

```


## ./pkg/interfaces/contracts/pool-hooks/IMevCaptureHook.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/standalone-utils/IBalancerContractRegistry.sol

/// @notice Registered contracts must be one of these types.
enum ContractType {
    OTHER, // a blank entry will have a 0-value type, and it's safest to return this in that case
    POOL_FACTORY,
    ROUTER,
    HOOK,
    ERC4626
}

interface IBalancerContractRegistry {
    /**
     * @notice Store the state of a registered Balancer contract.
     * @dev Contracts can be deprecated, so we store an active flag indicating the status. With two flags, we can
     * differentiate between deprecated and non-existent. The same contract address can have multiple names, but
     * only one type. If a contract is legitimately multiple types (e.g., a hook that also acts as a router), set
     * the type to its "primary" function: hook, in this case. The "Other" type is intended as a catch-all for
     * things that don't find into the standard types (e.g., helper contracts).
     *
     * @param contractType The type of contract (e.g., Router or Hook)
     * @param isRegistered This flag indicates whether there is an entry for the associated address
     * @param isActive If there is an entry, this flag indicates whether it is active or deprecated
     */
    struct ContractInfo {
        ContractType contractType;
        bool isRegistered;
        bool isActive;
    }

    /**
     * @notice Emitted when a new contract is registered.
     * @param contractType The type of contract being registered
     * @param contractName The name of the contract being registered
     * @param contractAddress The address of the contract being registered
     */
    event BalancerContractRegistered(
        ContractType indexed contractType,
        string indexed contractName,
        address indexed contractAddress
    );

    /**
     * @notice Emitted when a new contract is deregistered (deleted).
     * @param contractType The type of contract being deregistered
     * @param contractName The name of the contract being deregistered
     * @param contractAddress The address of the contract being deregistered
     */
    event BalancerContractDeregistered(
        ContractType indexed contractType,
        string indexed contractName,
        address indexed contractAddress
    );

    /**
     * @notice Emitted when a registered contract is deprecated.
     * @dev This sets the `isActive` flag to false.
     * @param contractAddress The address of the contract being deprecated
     */
    event BalancerContractDeprecated(address indexed contractAddress);

    /**
     * @notice Emitted when an alias is added or updated.
     * @param contractAlias The alias name
     * @param contractAddress The address of the contract being deprecated
     */
    event ContractAliasUpdated(string indexed contractAlias, address indexed contractAddress);

    /**
     * @notice A contract has already been registered under the given address.
     * @dev Both names and addresses must be unique in the primary registration mapping. Though there are two mappings
     * to accommodate searching by either name or address, conceptually there is a single guaranteed-consistent
     * name => address => state mapping.
     *
     * @param contractType The contract type, provided for documentation purposes
     * @param contractAddress The address of the previously registered contract
     */
    error ContractAddressAlreadyRegistered(ContractType contractType, address contractAddress);

    /**
     * @notice A contract has already been registered under the given name.
     * @dev Note that names must be unique; it is not possible to register two contracts with the same name and
     * different types, or the same name and different addresses.
     *
     * @param contractType The registered contract type, provided for documentation purposes
     * @param contractName The name of the previously registered contract
     */
    error ContractNameAlreadyRegistered(ContractType contractType, string contractName);

    /**
     * @notice The proposed contract name has already been added as an alias.
     * @dev This could lead to inconsistent (or at least redundant) internal state if allowed.
     * @param contractName The name of the previously registered contract
     * @param contractAddress The address of the previously registered contract
     */
    error ContractNameInUseAsAlias(string contractName, address contractAddress);

    /**
     * @notice The proposed alias has already been registered as a contract.
     * @dev This could lead to inconsistent (or at least redundant) internal state if allowed.
     * @param contractType The registered contract type, provided for documentation purposes
     * @param contractName The name of the previously registered contract (and proposed alias)
     */
    error ContractAliasInUseAsName(ContractType contractType, string contractName);

    /**
     * @notice Thrown when attempting to deregister a contract that was not previously registered.
     * @param contractName The name of the unregistered contract
     */
    error ContractNameNotRegistered(string contractName);

    /**
     * @notice An operation that requires a valid contract specified an unrecognized address.
     * @dev A contract being deprecated was never registered, or the target of an alias isn't a previously
     * registered contract.
     *
     * @param contractAddress The address of the contract that was not registered
     */
    error ContractAddressNotRegistered(address contractAddress);

    /**
     * @notice Contracts can only be deprecated once.
     * @param contractAddress The address of the previously deprecated contract
     */
    error ContractAlreadyDeprecated(address contractAddress);

    /// @notice Cannot register or deprecate contracts, or add an alias targeting the zero address.
    error ZeroContractAddress();

    /// @notice Cannot register (or deregister) a contract with an empty string as a name.
    error InvalidContractName();

    /// @notice Cannot add an empty string as an alias.
    error InvalidContractAlias();

    /**
     * @notice Register an official Balancer contract (e.g., a trusted router, standard pool factory, or hook).
     * @dev This is a permissioned function, and does only basic validation of the address (non-zero) and the name
     * (not blank). Governance must ensure this is called with valid information. Emits the
     * `BalancerContractRegistered` event if successful. Reverts if either the name or address is invalid or
     * already in use.
     *
     * @param contractType The type of contract being registered
     * @param contractName A text description of the contract, usually the deployed version (e.g., "v3-pool-weighted")
     * @param contractAddress The address of the contract
     */
    function registerBalancerContract(
        ContractType contractType,
        string memory contractName,
        address contractAddress
    ) external;

    /**
     * @notice Deregister an official Balancer contract (e.g., a trusted router, standard pool factory, or hook).
     * @dev This is a permissioned function, and makes it possible to correct errors without complex update logic.
     * If a contract was registered with an incorrect type, name, or address, this allows governance to simply delete
     * it, and register it again with the correct data. It must start with the name, as this is the registry key,
     * required for complete deletion.
     *
     * Note that there might still be an alias targeting the address being deleted, but accessing it will just return
     * inactive, and this orphan alias can simply be overwritten with `addOrUpdateBalancerContractAlias` to point to
     * the correct address.
     *
     * @param contractName The name of the contract being deprecated (cannot be an alias)
     */
    function deregisterBalancerContract(string memory contractName) external;

    /**
     * @notice Deprecate an official Balancer contract.
     * @dev This is a permissioned function that sets the `isActive` flag to false in the contract info. It uses the
     * address instead of the name for maximum clarity, and to avoid having to handle aliases. Addresses and names are
     * enforced unique, so either the name or address could be specified in principle.
     *
     * @param contractAddress The address of the contract being deprecated
     */
    function deprecateBalancerContract(address contractAddress) external;

    /**
     * @notice Add an alias for a registered contract.
     * @dev This is a permissioned function to support querying by a contract alias. For instance, we might create a
     * `WeightedPool` alias meaning the "latest" version of the `WeightedPoolFactory`, so that off-chain addresses don't
     * need to track specific versions. Once added, an alias can also be updated to point to a different address
     * (e.g., when migrating from the v2 to the v3 weighted pool).
     *
     * @param contractAlias An alternate name that can be used to fetch a contract address
     * @param existingContract The target address of the contract alias
     */
    function addOrUpdateBalancerContractAlias(string memory contractAlias, address existingContract) external;

    /**
     * @notice Determine whether an address is an official contract of the specified type.
     * @param contractType The type of contract
     * @param contractAddress The address of the contract
     * @return isActive True if the given address is a registered and active contract of the specified type
     */
    function isActiveBalancerContract(
        ContractType contractType,
        address contractAddress
    ) external view returns (bool isActive);

    /**
     * @notice Look up a registered contract by type and name.
     * @dev This could target a particular version (e.g. `20241205-v3-weighted-pool`), or a contract alias
     * (e.g., `WeightedPool`).
     *
     * @param contractType The type of the contract
     * @param contractName The name of the contract
     * @return contractAddress The address of the associated contract, if registered, or zero
     * @return isActive True if the contract was registered and not deprecated
     */
    function getBalancerContract(
        ContractType contractType,
        string memory contractName
    ) external view returns (address contractAddress, bool isActive);

    /**
     * @notice Look up complete information about a registered contract by address.
     * @param contractAddress The address of the associated contract
     * @return info ContractInfo struct corresponding to the address
     */
    function getBalancerContractInfo(address contractAddress) external view returns (ContractInfo memory info);

    /// @notice Returns `true` if the given address is an active contract under the ROUTER type.
    function isTrustedRouter(address router) external view returns (bool);
}

// pkg/interfaces/contracts/pool-hooks/IMevCaptureHook.sol

interface IMevCaptureHook {
    /// @notice The `BalancerContractRegistry` set in the constructor is invalid.
    error InvalidBalancerContractRegistry();

    /**
     * @notice The pool was not registered with the MEV Hook contract.
     * @param pool Address of the pool that should have been registered with MevCaptureHook
     */
    error MevCaptureHookNotRegisteredInPool(address pool);

    /**
     * @notice The new max MEV swap fee percentage is above the allowed absolute maximum.
     * @param feePercentage New fee percentage being set
     * @param maxFeePercentage Absolute maximum allowed
     */
    error MevSwapFeePercentageAboveMax(uint256 feePercentage, uint256 maxFeePercentage);

    /**
     * @notice The sender is already registered as MEV tax-exempt.
     * @param sender Sender that is already MEV tax-exempt
     */
    error MevTaxExemptSenderAlreadyAdded(address sender);

    /**
     * @notice The sender is not registered as MEV tax-exempt.
     * @param sender Sender that is not MEV tax-exempt
     */
    error SenderNotRegisteredAsMevTaxExempt(address sender);

    /**
     * @notice The MEV tax was globally enabled or disabled in the hook.
     * @param enabled The new value for mevTaxEnabled. If true, MEV tax will be charged
     */
    event MevTaxEnabledSet(bool enabled);

    /**
     * @notice Default MEV tax multiplier was set.
     * @dev Registered pools should set the multiplier using `setPoolMevTaxMultiplier`.
     * @param newDefaultMevTaxMultiplier The new value for defaultMevTaxMultiplier
     */
    event DefaultMevTaxMultiplierSet(uint256 newDefaultMevTaxMultiplier);

    /**
     * @notice The default MEV tax threshold was set.
     * @dev Registered pools should set the threshold using `setPoolMevTaxThreshold`.
     * @param newDefaultMevTaxThreshold The new value for defaultMevTaxThreshold
     */
    event DefaultMevTaxThresholdSet(uint256 newDefaultMevTaxThreshold);

    /**
     * @notice The maximum MEV swap fee percentage was set.
     * @param maxMevSwapFeePercentage The new value for maxMevSwapFeePercentage.
     */
    event MaxMevSwapFeePercentageSet(uint256 maxMevSwapFeePercentage);

    /**
     * @notice A pool's MEV tax multiplier was set.
     * @param pool The address of the pool where the multiplier has changed
     * @param newPoolMevTaxMultiplier The new value for the pool multiplier
     */
    event PoolMevTaxMultiplierSet(address pool, uint256 newPoolMevTaxMultiplier);

    /**
     * @notice The default MEV tax threshold was set.
     * @param pool The address of the pool where the threshold has changed
     * @param newPoolMevTaxThreshold The new value for the pool threshold
     */
    event PoolMevTaxThresholdSet(address pool, uint256 newPoolMevTaxThreshold);

    /**
     * @notice The sender was registered as MEV tax-exempt.
     * @param sender The address of the sender registered as MEV tax-exempt
     */
    event MevTaxExemptSenderAdded(address sender);

    /**
     * @notice The sender was removed from the list of MEV tax-exempt senders.
     * @param sender The address of the sender removed from the MEV tax-exempt list
     */
    event MevTaxExemptSenderRemoved(address sender);

    /// @notice Returns `BalancerContractRegistry`.
    function getBalancerContractRegistry() external view returns (IBalancerContractRegistry);

    /**
     * @notice Check whether the MEV Tax is enabled in the hook.
     * @dev If MEV Tax is disabled, all swaps will pay the static swap fee amount.
     * @return mevTaxEnabled True if the MEV Tax is enabled
     */
    function isMevTaxEnabled() external view returns (bool mevTaxEnabled);

    /// @notice Permissioned function to reversibly disable charging the MEV Tax in registered pools.
    function disableMevTax() external;

    /// @notice Permissioned function to enable charging the MEV Tax in registered pools.
    function enableMevTax() external;

    /**
     * @notice Returns the maximum MEV swap fee percentage returned by `onComputeDynamicSwapFeePercentage`.
     * @dev The absolute minimum is still the static swap fee percentage of the pool.
     * In other words:
     * - if `maxMevSwapFeePercentage > staticSwapFeePercentage`, then
     * `staticSwapFeePercentage <= computedFeePercentage <= maxMevSwapFeePercentage`
     * - if `maxMevSwapFeePercentage <= staticSwapFeePercentage, then `computedFeePercentage = maxMevSwapFeePercentage`
     */
    function getMaxMevSwapFeePercentage() external view returns (uint256);

    /**
     * @notice Permissioned function to set the maximum MEV swap fee percentage returned by
     * `onComputeDynamicSwapFeePercentage`.
     * @dev See `getMaxMevSwapFeePercentage` for reference; this maximum applies only when
     * `maxMevSwapFeePercentage > staticSwapFeePercentage`.
     * Capped by MAX_FEE_PERCENTAGE defined by the Vault.
     */
    function setMaxMevSwapFeePercentage(uint256 maxMevSwapFeePercentage) external;

    /**
     * @notice Fetch the default multiplier for the priority gas price.
     * @dev The MEV swap fee percentage is calculated as `mevTaxMultiplier * priorityGasPrice`, where priorityGasPrice
     * is defined as `transactionGasPrice - baseFee`. This leads to a trade-off that requires careful calibration of
     * the mevTaxMultiplier to incentivize both searchers and LPs.
     *
     * A higher mevTaxMultiplier will raise the swap fee for searchers and accrue more priority fees for LPs. However,
     * raising the mevTaxMultiplier too high may raise searchers' priority fees to levels more typical of retail addresses,
     * making it difficult for the contract to distinguish between them.
     *
     * @return defaultMevTaxMultiplier The default MEV Tax Multiplier
     */
    function getDefaultMevTaxMultiplier() external view returns (uint256 defaultMevTaxMultiplier);

    /**
     * @notice Permissioned function to set the default multiplier of the priority gas price.
     * @dev The multiplier is not validated or limited by any value and can assume any 18-decimal number. That's
     * because the multiplier value depends on the priority gas price used by searchers in a given moment for a
     * specific chain. However, the resulting swap fee percentage, given by `priorityGasPrice * multiplier`, is capped
     * at the lower end by the static swap fee, and at the upper end by the maximum swap fee percentage of the vault.
     * Therefore, a multiplier with value 0 will effectively disable the MEV tax, since the static swap fee will be
     * charged. Also, a very high multiplier will make the trader pay the maximum configured swap fee which can be
     * close to 100%, effectively disabling swaps.
     *
     * @param newDefaultMevTaxMultiplier 18-decimal used to calculate the MEV swap fee percentage
     */
    function setDefaultMevTaxMultiplier(uint256 newDefaultMevTaxMultiplier) external;

    /**
     * @notice Fetch the priority gas price multiplier of the given pool.
     * @dev When a pool is registered with the MEV Hook in the vault, the MEV Hook initializes the multiplier of the
     * pool to the defaultMevTaxMultiplier value. If the pool is not registered with the MEV Hook, it reverts with
     * error MevCaptureHookNotRegisteredForPool(pool).
     *
     * @param pool Address of the pool with the multiplier
     * @return poolMevTaxMultiplier The multiplier of the pool
     */
    function getPoolMevTaxMultiplier(address pool) external view returns (uint256 poolMevTaxMultiplier);

    /**
     * @notice Permissioned function to set the MEV tax multiplier of a pool, overriding the default value.
     * @dev The multiplier is not validated or limited by any value and can assume any 18-decimal number. That's
     * because the multiplier value depends on the priority gas price used by searchers in a given moment for a
     * specific chain. If the pool is not registered with the MEV Hook, it reverts with error
     * MevCaptureHookNotRegisteredForPool(pool). However, the resulting swap fee percentage, given by
     * `priorityGasPrice * multiplier`, is capped in the lower end by the static swap fee, and at the upper end by
     * the maximum swap fee percentage of the vault. Therefore, a multiplier with value 0 will effectively disable the
     * MEV tax, since the static swap fee will be charged. Also, a very high multiplier will make the trader pay the
     * maximum configured swap fee which can be close to 100%, effectively disabling swaps.
     *
     * @param pool Address of the pool with the multiplier
     * @param newPoolMevTaxMultiplier New multiplier to be set in a pool
     */
    function setPoolMevTaxMultiplier(address pool, uint256 newPoolMevTaxMultiplier) external;

    /**
     * @notice Fetch the default priority gas price threshold.
     * @dev The MEV swap fee percentage is only applied if the priority gas price, defined as
     * `transactionGasPrice - baseFee`, is greater than the threshold.
     *
     * @return defaultMevTaxThreshold The default MEV Tax Threshold
     */
    function getDefaultMevTaxThreshold() external view returns (uint256 defaultMevTaxThreshold);

    /**
     * @notice Permissioned function to set the default priority gas price threshold.
     * @dev The threshold can be any unsigned integer and represents the priority gas price, in wei. It's used to
     * check whether the priority gas price level corresponds to a retail or searcher swap. The threshold value is not
     * capped by any value, since it depends on the chain state. A very high threshold (above the priority gas price of
     * searchers in the chain) will disable the MEV tax and charge the static swap fee.
     *
     * @param newDefaultMevTaxThreshold The new default threshold
     */
    function setDefaultMevTaxThreshold(uint256 newDefaultMevTaxThreshold) external;

    /**
     * @notice Fetch the priority gas price threshold of the given pool.
     * @dev When a pool is registered with the MEV Hook in the vault, the MEV Hook initializes the multiplier of the
     * pool with the defaultMevTaxMultiplier value. If the pool is not registered with the MEV Hook, it reverts with
     * error MevCaptureHookNotRegisteredForPool(pool).
     *
     * @param pool Address of the pool with the multiplier
     * @return poolMevTaxThreshold The threshold of the pool
     */
    function getPoolMevTaxThreshold(address pool) external view returns (uint256 poolMevTaxThreshold);

    /**
     * @notice Permissioned function to set the threshold of a pool, overriding the current value.
     * @dev The threshold can be any unsigned integer and represents the priority gas price, in wei. It's used to
     * check whether the priority gas price level corresponds to a retail or searcher swap. The threshold value is not
     * capped by any value, since it depends on the chain state. If the pool is not registered with the MEV Hook, it
     * reverts with error MevCaptureHookNotRegisteredForPool(pool). A very high threshold (above the priority gas price of
     * searchers in the chain) will disable the MEV tax and charge the static swap fee.
     *
     * @param pool Address of the pool with the threshold
     * @param newPoolMevTaxThreshold The new threshold to be set in a pool
     */
    function setPoolMevTaxThreshold(address pool, uint256 newPoolMevTaxThreshold) external;

    /**
     * @notice Checks whether the sender is MEV tax-exempt.
     * @dev A MEV tax-exempt sender pays only the static swap fee percentage, regardless of the priority fee.
     * @param sender The sender being checked for MEV tax-exempt status
     * @return mevTaxExempt True if the sender is MEV tax-exempt
     */
    function isMevTaxExemptSender(address sender) external view returns (bool mevTaxExempt);

    /**
     * @notice Registers a list of senders as MEV tax-exempt senders.
     * @param senders Addresses of senders to be registered as MEV tax-exempt
     */
    function addMevTaxExemptSenders(address[] memory senders) external;

    /**
     * @notice Removes a list of senders from the list of MEV tax-exempt senders.
     * @param senders Addresses of senders to be removed from the MEV tax-exempt list
     */
    function removeMevTaxExemptSenders(address[] memory senders) external;
}

```


## ./pkg/interfaces/contracts/standalone-utils/IPoolSwapFeeHelper.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/standalone-utils/IPoolSwapFeeHelper.sol

/**
 * @notice Maintain a set of pools whose static swap fee percentages can be changed from here, vs. from the Vault.
 * @dev Governance can add a set of pools to this contract, then grant swap fee setting permission to accounts on this
 * contract, which allows greater granularity than setting the permission directly on the Vault.
 *
 * Note that governance must grant this contract permission to set swap fees from the Vault, and only pools that
 * allow governance to set fees can be added (i.e., they must not have swap managers).
 */
interface IPoolSwapFeeHelper {
    /**
     * @notice Cannot add a pool that has a swap manager.
     * @dev The swap manager is an exclusive role. If it is set to a non-zero value during pool registration,
     * only the swap manager can change the fee. This helper can only set fees on pools that allow governance
     * to grant this permission.
     *
     * @param pool Address of the pool being added
     */
    error PoolHasSwapManager(address pool);

    /***************************************************************************
                                    Manage Pools
    ***************************************************************************/

    /**
     * @notice Set the static swap fee percentage on a given pool.
     * @dev This is a permissioned function. Governance must grant this contract permission to call
     * `setStaticSwapFeePercentage` on the Vault. Note that since the swap manager is an exclusive role, the swap fee
     * cannot be changed by governance if it is set, and the pool cannot be added to the set.
     *
     * @param pool The address of the pool
     * @param swapFeePercentage The new swap fee percentage
     */
    function setStaticSwapFeePercentage(address pool, uint256 swapFeePercentage) external;
}

```


## ./pkg/interfaces/contracts/standalone-utils/IPoolHelperCommon.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/standalone-utils/IPoolHelperCommon.sol

/// @notice Common interface for helper functions that operate on a subset of pools.
interface IPoolHelperCommon {
    /**
     * @notice The owner created a new pool set.
     * @dev Pools are organized into separate sets, controlled by a manager, which can be changed independently.
     * @param poolSetId Id of the set with which the pool is associated
     * @param initialManager Address of the pool set manager
     */
    event PoolSetCreated(uint256 indexed poolSetId, address indexed initialManager);

    /**
     * @notice The owner destroyed a pool set.
     * @param poolSetId Id of the set with which the pool is associated
     * @param manager The address of the set's last manager
     */
    event PoolSetDestroyed(uint256 indexed poolSetId, address indexed manager);

    /**
     * @notice The owner added a pool to the given set.
     * @param pool Address of the pool that was added
     * @param poolSetId Id of the set with which the pool is associated
     */
    event PoolAddedToSet(address indexed pool, uint256 indexed poolSetId);

    /**
     * @notice The owner removed a pool from the given set.
     * @param poolSetId Id of the set with which the pool is associated
     * @param pool Address of the pool that was removed
     */
    event PoolRemovedFromSet(address indexed pool, uint256 indexed poolSetId);

    /**
     * @notice The current manager of a pool set transferred ownership to a new address.
     * @dev Managers can only control one pool set. Transfers to existing managers of other sets will revert.
     * @param poolSetId Id of the set with which the pool is associated
     * @param oldManager Address of the previous manager
     * @param newManager Address of the new manager
     */
    event PoolSetOwnershipTransferred(
        uint256 indexed poolSetId,
        address indexed oldManager,
        address indexed newManager
    );

    /**
     * @notice Cannot add a pool that is already there.
     * @param pool Address of the pool being added
     * @param poolSetId Id of the set with which the pool is associated
     */
    error PoolAlreadyInSet(address pool, uint256 poolSetId);

    /**
     * @notice Cannot remove a pool that was not added.
     * @param pool Address of the pool being removed
     * @param poolSetId Id of the set with which the pool is associated
     */
    error PoolNotInSet(address pool, uint256 poolSetId);

    /**
     * @notice Pool set id associated with an operation is invalid.
     * @dev This can mean the value is invalid, or it was never created, or it was destroyed.
     * @param poolSetId The id of the invalid set
     */
    error InvalidPoolSetId(uint256 poolSetId);

    /// @notice The initial manager of a pool set cannot be zero.
    error InvalidPoolSetManager();

    /**
     * @notice Pool set managers can only manage a single pool set.
     * @dev Otherwise, the contract would not be able to determine the correct pool set from the caller's address.
     * @param poolSetManager Address of the manager that is already assigned to another pool set id
     */
    error PoolSetManagerNotUnique(address poolSetManager);

    /// @notice Permissioned operations on pools can only be performed by the pool set manager.
    error SenderIsNotPoolSetManager();

    /**
     * @notice An index is beyond the current bounds of the set.
     * @param poolSetId Id of the set involved in the operation
     */
    error IndexOutOfBounds(uint256 poolSetId);

    /***************************************************************************
                                 Manage Pool Sets
    ***************************************************************************/

    /**
     * @notice Create a new set with an initial manager, optionally initialized with a set of pools.
     * @dev The `newPools` list can be empty, in which case this will only create the set. Pools can then be
     * added with `addPoolsToSet`, or removed with `removePoolsFromSet`. This is a permissioned function.
     * Only the current owner of the helper contract (e.g., Maxis) may create new sets. Also reverts if the
     * initial manager address is zero or already a manager of another pool set.
     *
     * @param initialManager Address of the account authorized to perform operations on the set
     * @param newPools Set of pools to add to the set
     */
    function createPoolSet(address initialManager, address[] memory newPools) external returns (uint256 poolSetId);

    /**
     * @notice Create a new empty set with an initial manager.
     * @dev Convenience function to create a pool set with no initial pools. Also reverts if the initial manager
     * address is zero or already a manager of another pool set.
     *
     * @param initialManager Address of the account authorized to perform operations on the set
     */
    function createPoolSet(address initialManager) external returns (uint256 poolSetId);

    /**
     * @notice Simple way to remove an entire set of pools from control of the helper function.
     * @dev This is a permissioned function. Only the current owner of the helper contract (e.g., Maxis)
     * may destroy sets, effectively removing control of any pools in the set from the associated manager.
     * Also reverts if the poolSetId is not valid.
     *
     * @param poolSetId Id of the set being destroyed
     */
    function destroyPoolSet(uint256 poolSetId) external;

    /**
     * @notice Transfer ownership of a pool set from the current manager to a new manager.
     * @dev This is a permissioned function. Only the current manager of a set can call this to set a new manager.
     * Since managers can only control a single set, there is no need to specify the id in the call. Note that this
     * is a one-step migration. If it is done incorrectly, effective control of the set is lost, and the owner of this
     * contract will need to destroy the old set and create a new one with the correct initial manager. Also reverts
     * if the new manager address is zero or already the manager of a pool set.
     *
     * @param newManager The address of the new manager
     */
    function transferPoolSetOwnership(address newManager) external;

    /***************************************************************************
                                   Manage Pools
    ***************************************************************************/

    /**
     * @notice Add pools to the set of pools controlled by this helper contract.
     * @dev This is a permissioned function. Only the current owner of the helper contract (e.g., Maxis)
     * may add pools to a set. Also reverts if the poolSetId is not valid.
     *
     * @param newPools List of pools to add
     * @param poolSetId Id of the set to which the new pools belong
     */
    function addPoolsToSet(uint256 poolSetId, address[] memory newPools) external;

    /**
     * @notice Remove pools from the set of pools controlled by this helper contract.
     * @dev This is a permissioned function. Only the current owner of the helper contract (e.g., Maxis)
     * may remove pools from a set. Also reverts if the poolSetId is not valid.
     *
     * @param pools List of pools to remove from the set
     * @param poolSetId Id of the set to which the pools belong
     */
    function removePoolsFromSet(uint256 poolSetId, address[] memory pools) external;

    /***************************************************************************
                                    Getters                                
    ***************************************************************************/

    /**
     * @notice Get the pool set id associated with the caller.
     * @return poolSetId The numeric pool set id, or zero if the caller is not a pool set manager
     */
    function getPoolSetIdForCaller() external view returns (uint256 poolSetId);

    /**
     * @notice Get the pool set id associated with a given manager address.
     * @return poolSetId The numeric pool set id, or zero if the address given is not a pool set manager
     */
    function getPoolSetIdForManager(address manager) external view returns (uint256 poolSetId);

    /**
     * @notice Get the number of pools associated with the given set.
     * @dev Needed to support pagination in case the set is too large to process in a single transaction.
     * Reverts if the poolSetId is not valid.
     *
     * @param poolSetId Id of the set containing the pools
     * @return poolCount The current number of pools in the set
     */
    function getPoolCountForSet(uint256 poolSetId) external view returns (uint256 poolCount);

    /**
     * @notice Check whether a poolSetId has been created.
     * @param poolSetId Id of the set containing the pools
     * @return isValid True if the poolSetId exists
     */
    function isValidPoolSetId(uint256 poolSetId) external view returns (bool isValid);

    /**
     * @notice Check whether a pool is in the set of pools.
     * @dev Reverts if the poolSetId is not valid.
     * @param pool Address of the pool to check
     * @param poolSetId Id of the set containing the pools
     * @return poolInSet True if the pool is in the given set, false otherwise
     */
    function isPoolInSet(address pool, uint256 poolSetId) external view returns (bool poolInSet);

    /**
     * @notice Get the full set of pools from a given set.
     * @dev Reverts if the poolSetId is not valid.
     * @param poolSetId Id of the set containing the pools
     * @return pools List of pools
     */
    function getAllPoolsInSet(uint256 poolSetId) external view returns (address[] memory pools);

    /**
     * @notice Get a range of pools from a given set.
     * @dev Indexes are 0-based and [start, end) (i.e., inclusive of `start`; exclusive of `end`).
     * Reverts if the poolSetId is not valid.
     *
     * @param poolSetId Id of the set containing the pools
     * @param from Start index
     * @param to End index
     * @return pools List of pools
     */
    function getPoolsInSet(uint256 poolSetId, uint256 from, uint256 to) external view returns (address[] memory pools);

    /**
     * @notice Utility function to predict the next pool set id.
     * @return nextPoolSetId The pool set id that will be used on the next call of `createPoolSet`
     */
    function getNextPoolSetId() external view returns (uint256 nextPoolSetId);

    /**
     * @notice Get the manager address associated with a given poolSetId.
     * @param poolSetId Id of the set containing the pools
     * @return manager The address of the manager of the given poolSetId, or zero if the poolSetId is unassigned
     */
    function getManagerForPoolSet(uint256 poolSetId) external view returns (address manager);
}

```


## ./pkg/interfaces/contracts/standalone-utils/IProtocolFeeHelper.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/standalone-utils/IProtocolFeeHelper.sol

/**
 * @notice Maintain a set of pools whose protocol fees can be set from this helper contract, vs. the fee controller.
 * @dev Governance can add a set of pools to this contract, then grant permission to call protocol swap- or yield-
 * setting functions here, which allows greater granularity than setting permissions directly on the fee controller.
 *
 * Note that governance must grant this contract permission to call the pool protocol fee setting functions on the
 * controller.
 */
interface IProtocolFeeHelper {
    /***************************************************************************
                                    Manage Pools
    ***************************************************************************/

    /**
     * @notice Set the protocol swap fee for a pool.
     * @dev This contract must be granted permission to set swap and yield protocol fees, but governance can
     * independently grant permission to call the swap and yield fee setters.
     *
     * @param pool The address of the pool
     * @param newProtocolSwapFeePercentage The new protocol swap fee percentage
     */
    function setProtocolSwapFeePercentage(address pool, uint256 newProtocolSwapFeePercentage) external;

    /**
     * @notice Set the protocol yield fee for a pool.
     * @dev This contract must be granted permission to set swap and yield protocol fees, but governance can
     * independently grant permission to call the swap and yield fee setters.
     *
     * @param pool The address of the pool
     * @param newProtocolYieldFeePercentage The new protocol yield fee percentage
     */
    function setProtocolYieldFeePercentage(address pool, uint256 newProtocolYieldFeePercentage) external;
}

```


## ./pkg/interfaces/contracts/standalone-utils/IHyperEVMRateProviderFactory.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/standalone-utils/IHyperEVMRateProvider.sol

interface IHyperEVMRateProvider {
    /**
     * @notice The index of the token on the Hyperliquid public API.
     * @return tokenIndex The index of the token on the Hyperliquid public API
     */
    function getTokenIndex() external view returns (uint32);

    /**
     * @notice The index of the pair to fetch the spot price, according to the Hyperliquid public API.
     * @dev Hypercore has an index that identifies a pair of tokens to fetch the spot price.
     * @return pairIndex The index of the pair to fetch the spot price, according to the Hyperliquid public API
     */
    function getPairIndex() external view returns (uint32);

    /**
     * @notice The spot price multiplier.
     * @dev Hypercore returns the spot price with a different number of decimals for each token. So, to make this rate
     * provider compatible with the vault, we need to scale the spot price to 18 decimals using this multiplier.
     * @return spotPriceMultiplier The spot price multiplier
     */
    function getSpotPriceMultiplier() external view returns (uint256 spotPriceMultiplier);
}

// pkg/interfaces/contracts/standalone-utils/IHyperEVMRateProviderFactory.sol

interface IHyperEVMRateProviderFactory {
    /**
     * @notice A new HyperEVM Rate Provider was created.
     * @param tokenIndex The index of the base collateralAssetIndex on the Hyperliquid public API
     * @param pairIndex The index of the pair to fetch the spot price, according to the Hyperliquid public API
     * @param rateProvider The address of the deployed rate provider
     */
    event RateProviderCreated(uint256 indexed tokenIndex, uint256 indexed pairIndex, address indexed rateProvider);

    /// @notice Emitted when the factory is disabled.
    event RateProviderFactoryDisabled();

    /**
     * @notice A rate provider already exists for the given token and pair.
     * @param tokenIndex The index of the base collateralAssetIndex on the Hyperliquid public API
     * @param pairIndex The index of the pair to fetch the spot price, according to the Hyperliquid public API
     * @param rateProvider The address of the deployed rate provider
     */
    error RateProviderAlreadyExists(uint32 tokenIndex, uint32 pairIndex, address rateProvider);

    /**
     * @notice The rate provider was not found for the given token and pair.
     * @param tokenIndex The index of the base collateralAssetIndex on the Hyperliquid public API
     * @param pairIndex The index of the pair to fetch the spot price, according to the Hyperliquid public API
     */
    error RateProviderNotFound(uint32 tokenIndex, uint32 pairIndex);

    /// @notice The factory is disabled.
    error RateProviderFactoryIsDisabled();

    /**
     * @notice Returns a number representing the rate provider version.
     * @return rateProviderVersion The rate provider version number
     */
    function getRateProviderVersion() external view returns (uint256 rateProviderVersion);

    /**
     * @notice Creates a new HyperEVM Rate Provider for the given token and pair.
     * @param tokenIndex The index of the base collateralAssetIndex on the Hyperliquid public API
     * @param pairIndex The index of the pair to fetch the spot price, according to the Hyperliquid public API
     * @return rateProvider The address of the deployed rate provider
     */
    function create(uint32 tokenIndex, uint32 pairIndex) external returns (IHyperEVMRateProvider rateProvider);

    /**
     * @notice Gets the rate provider for the given token and pair.
     * @dev Reverts if the rate provider was not found for the given token and pair.
     * @param tokenIndex The index of the base collateralAssetIndex on the Hyperliquid public API
     * @param pairIndex The index of the pair to fetch the spot price, according to the Hyperliquid public API
     * @return rateProvider The address of the rate provider for the given token and pair
     */
    function getRateProvider(
        uint32 tokenIndex,
        uint32 pairIndex
    ) external view returns (IHyperEVMRateProvider rateProvider);

    /**
     * @notice Checks whether the given rate provider was created by this factory.
     * @param rateProvider The rate provider to check
     * @return success True if the rate provider was created by this factory; false otherwise
     */
    function isRateProviderFromFactory(IHyperEVMRateProvider rateProvider) external view returns (bool success);

    /**
     * @notice Disables the rate provider factory.
     * @dev A disabled rate provider factory cannot create new rate providers and cannot be re-enabled. However,
     * already created rate providers are still usable. This is a permissioned function.
     */
    function disable() external;
}

```


## ./pkg/interfaces/contracts/standalone-utils/IBalancerContractRegistry.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/standalone-utils/IBalancerContractRegistry.sol

/// @notice Registered contracts must be one of these types.
enum ContractType {
    OTHER, // a blank entry will have a 0-value type, and it's safest to return this in that case
    POOL_FACTORY,
    ROUTER,
    HOOK,
    ERC4626
}

interface IBalancerContractRegistry {
    /**
     * @notice Store the state of a registered Balancer contract.
     * @dev Contracts can be deprecated, so we store an active flag indicating the status. With two flags, we can
     * differentiate between deprecated and non-existent. The same contract address can have multiple names, but
     * only one type. If a contract is legitimately multiple types (e.g., a hook that also acts as a router), set
     * the type to its "primary" function: hook, in this case. The "Other" type is intended as a catch-all for
     * things that don't find into the standard types (e.g., helper contracts).
     *
     * @param contractType The type of contract (e.g., Router or Hook)
     * @param isRegistered This flag indicates whether there is an entry for the associated address
     * @param isActive If there is an entry, this flag indicates whether it is active or deprecated
     */
    struct ContractInfo {
        ContractType contractType;
        bool isRegistered;
        bool isActive;
    }

    /**
     * @notice Emitted when a new contract is registered.
     * @param contractType The type of contract being registered
     * @param contractName The name of the contract being registered
     * @param contractAddress The address of the contract being registered
     */
    event BalancerContractRegistered(
        ContractType indexed contractType,
        string indexed contractName,
        address indexed contractAddress
    );

    /**
     * @notice Emitted when a new contract is deregistered (deleted).
     * @param contractType The type of contract being deregistered
     * @param contractName The name of the contract being deregistered
     * @param contractAddress The address of the contract being deregistered
     */
    event BalancerContractDeregistered(
        ContractType indexed contractType,
        string indexed contractName,
        address indexed contractAddress
    );

    /**
     * @notice Emitted when a registered contract is deprecated.
     * @dev This sets the `isActive` flag to false.
     * @param contractAddress The address of the contract being deprecated
     */
    event BalancerContractDeprecated(address indexed contractAddress);

    /**
     * @notice Emitted when an alias is added or updated.
     * @param contractAlias The alias name
     * @param contractAddress The address of the contract being deprecated
     */
    event ContractAliasUpdated(string indexed contractAlias, address indexed contractAddress);

    /**
     * @notice A contract has already been registered under the given address.
     * @dev Both names and addresses must be unique in the primary registration mapping. Though there are two mappings
     * to accommodate searching by either name or address, conceptually there is a single guaranteed-consistent
     * name => address => state mapping.
     *
     * @param contractType The contract type, provided for documentation purposes
     * @param contractAddress The address of the previously registered contract
     */
    error ContractAddressAlreadyRegistered(ContractType contractType, address contractAddress);

    /**
     * @notice A contract has already been registered under the given name.
     * @dev Note that names must be unique; it is not possible to register two contracts with the same name and
     * different types, or the same name and different addresses.
     *
     * @param contractType The registered contract type, provided for documentation purposes
     * @param contractName The name of the previously registered contract
     */
    error ContractNameAlreadyRegistered(ContractType contractType, string contractName);

    /**
     * @notice The proposed contract name has already been added as an alias.
     * @dev This could lead to inconsistent (or at least redundant) internal state if allowed.
     * @param contractName The name of the previously registered contract
     * @param contractAddress The address of the previously registered contract
     */
    error ContractNameInUseAsAlias(string contractName, address contractAddress);

    /**
     * @notice The proposed alias has already been registered as a contract.
     * @dev This could lead to inconsistent (or at least redundant) internal state if allowed.
     * @param contractType The registered contract type, provided for documentation purposes
     * @param contractName The name of the previously registered contract (and proposed alias)
     */
    error ContractAliasInUseAsName(ContractType contractType, string contractName);

    /**
     * @notice Thrown when attempting to deregister a contract that was not previously registered.
     * @param contractName The name of the unregistered contract
     */
    error ContractNameNotRegistered(string contractName);

    /**
     * @notice An operation that requires a valid contract specified an unrecognized address.
     * @dev A contract being deprecated was never registered, or the target of an alias isn't a previously
     * registered contract.
     *
     * @param contractAddress The address of the contract that was not registered
     */
    error ContractAddressNotRegistered(address contractAddress);

    /**
     * @notice Contracts can only be deprecated once.
     * @param contractAddress The address of the previously deprecated contract
     */
    error ContractAlreadyDeprecated(address contractAddress);

    /// @notice Cannot register or deprecate contracts, or add an alias targeting the zero address.
    error ZeroContractAddress();

    /// @notice Cannot register (or deregister) a contract with an empty string as a name.
    error InvalidContractName();

    /// @notice Cannot add an empty string as an alias.
    error InvalidContractAlias();

    /**
     * @notice Register an official Balancer contract (e.g., a trusted router, standard pool factory, or hook).
     * @dev This is a permissioned function, and does only basic validation of the address (non-zero) and the name
     * (not blank). Governance must ensure this is called with valid information. Emits the
     * `BalancerContractRegistered` event if successful. Reverts if either the name or address is invalid or
     * already in use.
     *
     * @param contractType The type of contract being registered
     * @param contractName A text description of the contract, usually the deployed version (e.g., "v3-pool-weighted")
     * @param contractAddress The address of the contract
     */
    function registerBalancerContract(
        ContractType contractType,
        string memory contractName,
        address contractAddress
    ) external;

    /**
     * @notice Deregister an official Balancer contract (e.g., a trusted router, standard pool factory, or hook).
     * @dev This is a permissioned function, and makes it possible to correct errors without complex update logic.
     * If a contract was registered with an incorrect type, name, or address, this allows governance to simply delete
     * it, and register it again with the correct data. It must start with the name, as this is the registry key,
     * required for complete deletion.
     *
     * Note that there might still be an alias targeting the address being deleted, but accessing it will just return
     * inactive, and this orphan alias can simply be overwritten with `addOrUpdateBalancerContractAlias` to point to
     * the correct address.
     *
     * @param contractName The name of the contract being deprecated (cannot be an alias)
     */
    function deregisterBalancerContract(string memory contractName) external;

    /**
     * @notice Deprecate an official Balancer contract.
     * @dev This is a permissioned function that sets the `isActive` flag to false in the contract info. It uses the
     * address instead of the name for maximum clarity, and to avoid having to handle aliases. Addresses and names are
     * enforced unique, so either the name or address could be specified in principle.
     *
     * @param contractAddress The address of the contract being deprecated
     */
    function deprecateBalancerContract(address contractAddress) external;

    /**
     * @notice Add an alias for a registered contract.
     * @dev This is a permissioned function to support querying by a contract alias. For instance, we might create a
     * `WeightedPool` alias meaning the "latest" version of the `WeightedPoolFactory`, so that off-chain addresses don't
     * need to track specific versions. Once added, an alias can also be updated to point to a different address
     * (e.g., when migrating from the v2 to the v3 weighted pool).
     *
     * @param contractAlias An alternate name that can be used to fetch a contract address
     * @param existingContract The target address of the contract alias
     */
    function addOrUpdateBalancerContractAlias(string memory contractAlias, address existingContract) external;

    /**
     * @notice Determine whether an address is an official contract of the specified type.
     * @param contractType The type of contract
     * @param contractAddress The address of the contract
     * @return isActive True if the given address is a registered and active contract of the specified type
     */
    function isActiveBalancerContract(
        ContractType contractType,
        address contractAddress
    ) external view returns (bool isActive);

    /**
     * @notice Look up a registered contract by type and name.
     * @dev This could target a particular version (e.g. `20241205-v3-weighted-pool`), or a contract alias
     * (e.g., `WeightedPool`).
     *
     * @param contractType The type of the contract
     * @param contractName The name of the contract
     * @return contractAddress The address of the associated contract, if registered, or zero
     * @return isActive True if the contract was registered and not deprecated
     */
    function getBalancerContract(
        ContractType contractType,
        string memory contractName
    ) external view returns (address contractAddress, bool isActive);

    /**
     * @notice Look up complete information about a registered contract by address.
     * @param contractAddress The address of the associated contract
     * @return info ContractInfo struct corresponding to the address
     */
    function getBalancerContractInfo(address contractAddress) external view returns (ContractInfo memory info);

    /// @notice Returns `true` if the given address is an active contract under the ROUTER type.
    function isTrustedRouter(address router) external view returns (bool);
}

```


## ./pkg/interfaces/contracts/standalone-utils/IPoolPauseHelper.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/standalone-utils/IPoolPauseHelper.sol

/**
 * @notice Maintain a set of pools that can be paused from this helper contract, vs. directly from the Vault.
 * @dev Governance can add a set of pools to this contract, then grant pause permission to accounts here, which
 * allows greater granularity than setting the permission directly on the Vault.
 *
 * Note that governance must grant this contract permission to pause pools from the Vault. Unpausing is not
 * addressed here, and must still be done through the Vault.
 */
interface IPoolPauseHelper {
    /**
     * @notice Pause a set of pools.
     * @dev This is a permissioned function. Governance must first grant this contract permission to call `pausePool`
     * on the Vault, then grant another account permission to call `pausePools` here. Note that this is not necessarily
     * the same account that can add or remove pools from the pausable list.
     *
     * Note that there is no `unpause`. This is a helper contract designed to react quickly to emergencies. Unpausing
     * is a more deliberate action that should be performed by accounts approved by governance for this purpose, or by
     * the individual pools' pause managers.
     *
     * @param pools List of pools to pause
     */
    function pausePools(address[] memory pools) external;
}

```


## ./pkg/interfaces/contracts/standalone-utils/IHyperEVMRateProvider.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/standalone-utils/IHyperEVMRateProvider.sol

interface IHyperEVMRateProvider {
    /**
     * @notice The index of the token on the Hyperliquid public API.
     * @return tokenIndex The index of the token on the Hyperliquid public API
     */
    function getTokenIndex() external view returns (uint32);

    /**
     * @notice The index of the pair to fetch the spot price, according to the Hyperliquid public API.
     * @dev Hypercore has an index that identifies a pair of tokens to fetch the spot price.
     * @return pairIndex The index of the pair to fetch the spot price, according to the Hyperliquid public API
     */
    function getPairIndex() external view returns (uint32);

    /**
     * @notice The spot price multiplier.
     * @dev Hypercore returns the spot price with a different number of decimals for each token. So, to make this rate
     * provider compatible with the vault, we need to scale the spot price to 18 decimals using this multiplier.
     * @return spotPriceMultiplier The spot price multiplier
     */
    function getSpotPriceMultiplier() external view returns (uint256 spotPriceMultiplier);
}

```


## ./pkg/interfaces/contracts/vault/ICompositeLiquidityRouterErrors.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/vault/ICompositeLiquidityRouterErrors.sol

/// @notice Errors are declared inside an interface (namespace) to improve DX with Typechain.
interface ICompositeLiquidityRouterErrors {
    /**
     * @notice The actual result of the liquidity removal operation does not match the expected set of tokens.
     * @param actualTokensOut The set of tokens derived from pool traversal
     * @param expectedTokensOut The set of tokens supplied by the user
     */
    error WrongTokensOut(address[] actualTokensOut, address[] expectedTokensOut);

    /**
     * @notice The `tokensIn` array contains a duplicate token.
     * @dev Note that it's technically possible to have duplicate tokens with 0 amounts, as those are ignored.
     * @param duplicateToken The address of the duplicate token
     */
    error DuplicateTokenIn(address duplicateToken);
}

```


## ./pkg/interfaces/contracts/vault/ITimelockAuthorizer.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/vault/ITimelockAuthorizer.sol

/**
 * @title Timelock Authorizer
 * @author Balancer Labs
 * @dev Authorizer with timelocks (delays).
 *
 * Users are allowed to perform actions if they have the permission to do so.
 *
 * This Authorizer implementation allows defining delays per action identifier. If a delay is set for an action, addresses
 * are not allowed to execute it directly themselves. Instead, they schedule an execution that the Authorizer will
 * run in the future.
 *
 * Glossary:
 * - Action: Operation that can be performed on a target contract. These are identified by a unique bytes32 `actionId`
 *   defined by each target contract following `IAuthentication.getActionId`.
 * - Scheduled execution: The Authorizer can define a delay for an `actionId` to require that a specific
 *   time window must pass before it can be executed. When a delay is set for an `actionId`, executions
 *   must be scheduled. These executions are identified by an unsigned integer called `scheduledExecutionId`.
 * - Permission: Accounts have or don't have permission to perform an action identified by its `actionId` on a specific
 *   contract `where`. Note that if the action has a delay, then accounts with permission cannot perform the action
 *   directly, but are instead allowed to schedule future executions for them.
 *
 * Note that the TimelockAuthorizer doesn't use reentrancy guard on its external functions.
 * The only function which makes an external non-view call (and so could initiate a reentrancy attack) is `execute`
 * which executes a scheduled execution, protected by the Checks-Effects-Interactions pattern.
 * In fact a number of the TimelockAuthorizer's functions may only be called through a scheduled execution so reentrancy
 * is necessary in order to be able to call these.
 */
interface ITimelockAuthorizer {
    struct ScheduledExecution {
        address where;
        bytes data;
        bool executed;
        bool canceled;
        bool protected;
        uint256 executableAt;
        address scheduledBy;
        uint256 scheduledAt;
        address executedBy;
        uint256 executedAt;
        address canceledBy;
        uint256 canceledAt;
    }

    /// @notice Emitted when a root change is scheduled.
    event RootChangeScheduled(address indexed newRoot, uint256 indexed scheduledExecutionId);

    /// @notice Emitted when an executor is added for a scheduled execution `scheduledExecutionId`.
    event ExecutorAdded(uint256 indexed scheduledExecutionId, address indexed executor);

    /// @notice Emitted when an account is added as a granter for `actionId` in `where`.
    event GranterAdded(bytes32 indexed actionId, address indexed account, address indexed where);

    /// @notice Emitted when an account is removed as a granter `actionId` in `where`.
    event GranterRemoved(bytes32 indexed actionId, address indexed account, address indexed where);

    /// @notice Emitted when `account` is added as a revoker in `where`.
    event RevokerAdded(address indexed account, address indexed where);

    /// @notice Emitted when an account is removed as a revoker in `where`.
    event RevokerRemoved(address indexed account, address indexed where);

    /// @notice Emitted when a canceler is added for a scheduled execution `scheduledExecutionId`.
    event CancelerAdded(uint256 indexed scheduledExecutionId, address indexed canceler);

    /// @notice Emitted when a canceler is removed for a scheduled execution `scheduledExecutionId`.
    event CancelerRemoved(uint256 indexed scheduledExecutionId, address indexed canceler);

    /// @notice Emitted when an execution `scheduledExecutionId` is executed.
    event ExecutionExecuted(uint256 indexed scheduledExecutionId);

    /// @notice Emitted when an execution `scheduledExecutionId` is canceled.
    event ExecutionCanceled(uint256 indexed scheduledExecutionId);

    /// @notice Emitted when a new `root` is set.
    event RootSet(address indexed root);

    /// @notice Emitted when a new `pendingRoot` is set. The new account must claim ownership for it to take effect.
    event PendingRootSet(address indexed pendingRoot);

    /// @notice Emitted when a revoke permission is scheduled.
    event RevokePermissionScheduled(
        bytes32 indexed actionId,
        address indexed account,
        address indexed where,
        uint256 scheduledExecutionId
    );

    /// @notice Emitted when a grant permission is scheduled.
    event GrantPermissionScheduled(
        bytes32 indexed actionId,
        address indexed account,
        address indexed where,
        uint256 scheduledExecutionId
    );

    /// @notice Emitted when a revoke delay change is scheduled.
    event RevokeDelayChangeScheduled(
        bytes32 indexed actionId,
        uint256 indexed newDelay,
        uint256 indexed scheduledExecutionId
    );

    /// @notice Emitted when a grant delay change is scheduled.
    event GrantDelayChangeScheduled(
        bytes32 indexed actionId,
        uint256 indexed newDelay,
        uint256 indexed scheduledExecutionId
    );

    /// @notice Emitted when a delay change is scheduled.
    event DelayChangeScheduled(
        bytes32 indexed actionId,
        uint256 indexed newDelay,
        uint256 indexed scheduledExecutionId
    );

    /// @notice Emitted when a new `delay` is set in order to perform action `actionId`.
    event ActionDelaySet(bytes32 indexed actionId, uint256 delay);

    /// @notice Emitted when a new execution `scheduledExecutionId` is scheduled.
    event ExecutionScheduled(bytes32 indexed actionId, uint256 indexed scheduledExecutionId);

    /// @notice Emitted when a new `delay` is set in order to grant permission to execute action `actionId`.
    event GrantDelaySet(bytes32 indexed actionId, uint256 delay);

    /// @notice Emitted when a new `delay` is set in order to revoke permission to execute action `actionId`.
    event RevokeDelaySet(bytes32 indexed actionId, uint256 delay);

    /// @notice Emitted when `account` is granted permission to perform action `actionId` in target `where`.
    event PermissionGranted(bytes32 indexed actionId, address indexed account, address indexed where);

    /// @notice Emitted when `account`'s permission to perform action `actionId` in target `where` is revoked.
    event PermissionRevoked(bytes32 indexed actionId, address indexed account, address indexed where);

    // solhint-disable func-name-mixedcase

    /**
     * @notice A constant value for `scheduledExecutionId` that will match any execution Id.
     * Cancelers assigned to this Id will be able to cancel *any* scheduled execution,
     * which is very useful for e.g. emergency response dedicated teams that analyze these.
     */
    function GLOBAL_CANCELER_SCHEDULED_EXECUTION_ID() external view returns (uint256);

    /**
     * @notice A sentinel value for `where` that will match any address.
     */
    function EVERYWHERE() external view returns (address);

    /**
     * @notice We institute a maximum delay to ensure that actions cannot be accidentally/maliciously disabled through
     * setting an arbitrarily long delay.
     */
    function MAX_DELAY() external view returns (uint256);

    /// @notice We need a minimum delay period to ensure that all delay changes may be properly scrutinized.
    function MINIMUM_CHANGE_DELAY_EXECUTION_DELAY() external view returns (uint256);

    /// @notice Returns true if `account` is the root.
    function isRoot(address account) external view returns (bool);

    /// @notice Returns true if `account` is the pending root.
    function isPendingRoot(address account) external view returns (bool);

    /// @notice Returns the delay required to transfer the root address.
    function getRootTransferDelay() external view returns (uint256);

    /// @notice Returns the vault address.
    function getVault() external view returns (address);

    /// @notice Returns the TimelockExecutionHelper address.
    function getTimelockExecutionHelper() external view returns (address);

    /// @notice Returns the root address.
    function getRoot() external view returns (address);

    /// @notice Returns the currently pending new root address.
    function getPendingRoot() external view returns (address);

    /// @notice Returns true if `account` is allowed to grant permissions for action `actionId` in target `where`.
    function isGranter(bytes32 actionId, address account, address where) external view returns (bool);

    /// @notice Returns true if `account` is allowed to revoke permissions in target `where` for all actions.
    function isRevoker(address account, address where) external view returns (bool);

    /// @notice Returns the scheduled execution `scheduledExecutionId`.
    function getScheduledExecution(uint256 scheduledExecutionId) external view returns (ScheduledExecution memory);

    /**
     * @notice Returns the lifetime count of scheduled executions. The most recent scheduled execution will always have
     * a `scheduledExecutionId` of `getScheduledExecutionsCount() - 1`
     */
    function getScheduledExecutionsCount() external view returns (uint256);

    /**
     * @notice Returns multiple scheduled executions, in either chronological or reverse chronological order (if
     * `reverseOrder` is true).
     *
     * This function will return at most `maxSize` items, starting at index `skip` (meaning the first entries are
     * skipped). Note that when querying in reverse order, it is the newest entries that are skipped, not the oldest.
     *
     * The value of `skip` must be lower than the return value of `getScheduledExecutionsCount()`, which means that not
     * all scheduled executions can be skipped, and at least one execution will always be returned (assuming there are
     * any).
     *
     * Example calls:
     *  - { skip: 0, reverseOrder: false } : returns up to `maxSize` of oldest entries, with the oldest at index 0
     *  - { skip: 0, reverseOrder: true } : returns up to `maxSize` of the newest entries, with the newest at index 0
     *  - { skip: 5, reverseOrder: false } : returns up to `maxSize` of the oldest entries, skipping the 5 oldest
     *    entries, with the globally sixth oldest at index 0
     *  - { skip: 5, reverseOrder: true } : returns up to `maxSize` of the newest entries, skipping the 5 newest
     *    entries, with the globally sixth newest at index 0
     */
    function getScheduledExecutions(
        uint256 skip,
        uint256 maxSize,
        bool reverseOrder
    ) external view returns (ITimelockAuthorizer.ScheduledExecution[] memory items);

    /// @notice Returns true if `account` is an executor for `scheduledExecutionId`.
    function isExecutor(uint256 scheduledExecutionId, address account) external view returns (bool);

    /**
     * @notice Returns true if execution `scheduledExecutionId` can be executed.
     * @dev Only true if it is not already executed or canceled, and if the execution delay has passed.
     */
    function canExecute(uint256 scheduledExecutionId) external view returns (bool);

    /// @notice Returns true if `account` is an canceler for `scheduledExecutionId`.
    function isCanceler(uint256 scheduledExecutionId, address account) external view returns (bool);

    /// @notice Schedules an execution to change the root address to `newRoot`.
    function scheduleRootChange(address newRoot, address[] memory executors) external returns (uint256);

    /// @notice Returns the execution delay for action `actionId`.
    function getActionIdDelay(bytes32 actionId) external view returns (uint256);

    /// @notice Returns the execution delay for granting permission for action `actionId`.
    function getActionIdGrantDelay(bytes32 actionId) external view returns (uint256);

    /// @notice Returns the execution delay for revoking permission for action `actionId`.
    function getActionIdRevokeDelay(bytes32 actionId) external view returns (uint256);

    /**
     * @notice Returns true if `account` has the permission defined by action `actionId` and target `where`.
     * @dev This function is specific for the strict permission defined by the tuple `(actionId, where)`: `account` may
     * instead hold the global permission for the action `actionId`, also granting them permission on `where`, but this
     * function would return false regardless.
     *
     * For this reason, it's recommended to use `hasPermission` if checking whether `account` is allowed to perform
     * a given action.
     */
    function isPermissionGrantedOnTarget(bytes32 actionId, address account, address where) external view returns (bool);

    /// @notice Returns true if `account` has permission over the action `actionId` in target `where`.
    function hasPermission(bytes32 actionId, address account, address where) external view returns (bool);

    /**
     * @notice Sets the pending root address to `pendingRoot`.
     * @dev This function can never be called directly - it is only ever called as part of a scheduled execution by
     * the TimelockExecutionHelper after after calling `scheduleRootChange`.
     *
     * Once set as the pending root, `pendingRoot` may then call `claimRoot` to become the new root.
     */
    function setPendingRoot(address pendingRoot) external;

    /**
     * @notice Transfers root powers from the current to the pending root address.
     * @dev This function prevents accidentally transferring root to an invalid address.
     * To become root, the pending root must call this function to ensure that it's able to interact with this contract.
     */
    function claimRoot() external;

    /**
     * @notice Executes a scheduled execution `scheduledExecutionId`. This is used to execute all scheduled executions,
     * not only those that originate from `schedule`, but also internal TimelockAuthorizer functions such as
     * `scheduleRootChange` or `scheduleDelayChange`.
     *
     * If any executors were set up when scheduling, `execute` can only be called by them. If none were set, the
     * scheduled execution is said to be 'unprotected', and can be executed by anyone.
     *
     * Once executed, a scheduled execution cannot be executed again. It also cannot be executed if canceled.
     *
     * We mark this function as `nonReentrant` out of an abundance of caution, as in theory this and the Authorizer
     * should be resilient to reentrant executions. The non-reentrancy check means that it is not possible to execute a
     * scheduled execution during the execution of another scheduled execution - an unlikely and convoluted scenario
     * that we explicitly forbid.
     *
     * Note that while `execute` is nonReentrant, other functions are not - indeed, we rely on reentrancy to e.g. call
     * `setPendingRoot` or `setDelay`.
     */
    function execute(uint256 scheduledExecutionId) external returns (bytes memory result);

    /**
     * @notice Cancels a scheduled execution `scheduledExecutionId`, which prevents execution via `execute`. Canceling
     * is irreversible. Scheduled executions that have already been executed cannot be canceled. This is the only way to
     * prevent a scheduled execution from being executed (assuming there are willing executors).
     *
     * The caller must be a canceler, a permission which is managed by the `addCanceler` and `removeCanceler` functions.
     * Note that root is always a canceler for all scheduled executions.
     */
    function cancel(uint256 scheduledExecutionId) external;

    /**
     * @notice Grants canceler status to `account` for scheduled execution `scheduledExecutionId`.
     * @dev Only the root can add a canceler.
     *
     * Note that there are no delays associated with adding or removing cancelers. This is based on the assumption that
     * any action which a malicious address could exploit to damage the protocol can be mitigated by root.
     * Root can remove any canceler and reschedule any task
     */
    function addCanceler(uint256 scheduledExecutionId, address account) external;

    /**
     * @notice Remove canceler status from `account` for scheduled execution `scheduledExecutionId`.
     * @dev Only the root can remove a canceler.
     */
    function removeCanceler(uint256 scheduledExecutionId, address account) external;

    /**
     * @notice Grants granter status to `account` for action `actionId` in target `where`.
     * @dev Only the root can add granters.
     *
     * Note that there are no delays associated with adding or removing granters. This is based on the assumption that
     * any action which a malicious address could exploit to damage the protocol will have a sufficiently long delay
     * associated with either granting permission for or exercising that permission such that the root will be able to
     * reestablish control and cancel either the granting or associated action before it can be executed, and then
     * remove the granter.
     *
     * A malicious granter may also attempt to use their granter status to grant permission to multiple accounts, but
     * they cannot add new granters. Therefore, the danger posed by a malicious granter is limited and self-
     * contained. Root can mitigate the situation simply and completely by revoking first their granter status,
     * and then any permissions granted by that account, knowing there cannot be any more.
     */
    function addGranter(bytes32 actionId, address account, address where) external;

    /**
     * @notice Revokes granter status from `account` for action `actionId` in target `where`.
     * @dev Only the root can remove granters.
     *
     * Note that there are no delays associated with removing granters. The only instance in which one might be useful
     * is if we had contracts that were granters, and this was depended upon for operation of the system. This however
     * doesn't seem like it will ever be required - granters are typically subDAOs.
     *
     * After removing a malicious granter, care should be taken to review their actions and remove any permissions
     * granted by them, or cancel scheduled grants. This should be done *after* removing the granter, at which point
     * they won't be able to create any more of these.
     */
    function removeGranter(bytes32 actionId, address account, address where) external;

    /**
     * @notice Grants revoker status to `account` in target `where` for all actions.
     * @dev Only the root can add revokers.
     *
     * Note that there are no delays associated with adding revokers. This is based on the assumption that any
     * permissions for which revocation from key addresses would be dangerous (e.g. preventing the BalancerMinter from
     * minting BAL) have sufficiently long delays associated with revoking them that the root will be able to
     * reestablish control and cancel the revocation before the scheduled revocation can be executed.
     *
     * A malicious revoker cannot add new revokers, so root can simply revoke their status once.
     */
    function addRevoker(address account, address where) external;

    /**
     * @notice Removes revoker status from `account` in target `where` for all actions.
     * @dev Only the root can remove revokers.
     *
     * Note that there are no delays associated with removing revokers.  The only instance in which one might be useful
     * is if we had contracts that were revoker, and this was depended upon for operation of the system. This however
     * doesn't seem like it will ever be required - revokers are typically subDAOs.
     */
    function removeRevoker(address account, address where) external;

    /**
     * @notice Sets a new delay `delay` for action `actionId`.
     * @dev This function can never be called directly - it is only ever called as part of a scheduled execution by
     * the TimelockExecutionHelper after after calling `scheduleDelayChange`.
     */
    function setDelay(bytes32 actionId, uint256 delay) external;

    /**
     * @notice Sets a new grant action delay `delay` for action `actionId`
     * @dev This function can never be called directly - it is only ever called as part of a scheduled execution by
     * the TimelockExecutor after after calling `scheduleGrantDelayChange`.
     * Delay has to be shorter than the Authorizer delay.
     */
    function setGrantDelay(bytes32 actionId, uint256 delay) external;

    /**
     * @notice Sets a new revoke action delay `delay` for action `actionId`
     * @dev This function can never be called directly - it is only ever called as part of a scheduled execution by
     * the TimelockExecutor after after calling `scheduleRevokeDelayChange`.
     * Delay has to be shorter than the Authorizer delay.
     */
    function setRevokeDelay(bytes32 actionId, uint256 delay) external;

    /**
     * @notice Schedules an execution to set the delay for `actionId`' to `newDelay`. This makes it impossible to
     * execute `actionId` without an immutable public on-chain commitment for the execution at least `newDelay` seconds
     * in advance.
     *
     * Critical actions that are expected to be performed by EOAs or multisigs are typically subject to such delays to
     * allow for public scrutiny.
     *
     * How long it will take to make this change will depend on the current and new delays: if increasing by more than
     * 5 days, then the time difference between the delays must pass. Otherwise, the minimum delay change execution
     * delay of 5 days must pass instead.
     *
     * Only `executors` will be able to execute the scheduled execution, unless `executors` is an empty array, in which
     * case any account can execute it.
     *
     * Avoid scheduling multiple delay changes for the same action at the same time, as this makes it harder to reason
     * about the state of the system. If there is already a scheduled delay change and there is a desire to change the
     * future delay to some other value, cancel the first scheduled change and schedule a new one.
     *
     * Only root can call this function, but other accounts may be granted permission to cancel the scheduled execution
     * (including global cancelers).
     */
    function scheduleDelayChange(
        bytes32 actionId,
        uint256 newDelay,
        address[] memory executors
    ) external returns (uint256);

    /**
     * @notice Schedules an execution to set the delay for granting permission over `actionId` to `newDelay`. This makes
     * it impossible to grant permission to execute `actionId` without an immutable public on-chain commitment for the
     * granting at least `newDelay` seconds in advance.
     *
     * Critical actions that are expected to be performed by smart contracts are typically subject to such grant delays
     * to allow for public scrutiny of new contracts that are granted the permission.
     *
     * How long it will take to make this change will depend on the current and new grant delays: if increasing by more
     * than 5 days, then the time difference between the grant delays must pass. Otherwise, the minimum delay change
     * execution delay of 5 days must pass instead.
     *
     * Only `executors` will be able to execute the scheduled execution, unless `executors` is an empty array, in which
     * case any account can execute it.
     *
     * Avoid scheduling multiple grant delay changes for the same action at the same time, as this makes it harder to
     * reason about the state of the system. If there is already a scheduled grant delay change and there is a desire to
     * change the future grant delay to some other value, cancel the first scheduled change and schedule a new one.
     *
     * Only root can call this function, but other accounts may be granted permission to cancel the scheduled execution
     * (including global cancelers).
     */
    function scheduleGrantDelayChange(
        bytes32 actionId,
        uint256 newDelay,
        address[] memory executors
    ) external returns (uint256);

    /**
     * @notice Schedules an execution to set the delay for revoking permission over `actionId` to `newDelay`. This makes
     * it impossible to revoke permission to execute `actionId` without an immutable public on-chain commitment for the
     * revoking at least `newDelay` seconds in advance.
     *
     * Critical actions that are performed by smart contracts and to which there is a long term commitment (e.g. minting
     * of BAL as part of the Liquidity Mining Program) are typically subject to such revoke delays, making it impossible
     * to disable the system without sufficient notice.
     *
     * How long it will take to make this change will depend on the current and new revoke delays: if increasing by more
     * than 5 days, then the time difference between the revoke delays must pass. Otherwise, the minimum delay change
     * execution delay of 5 days must pass instead.
     *
     * Only `executors` will be able to execute the scheduled execution, unless `executors` is an empty array, in which
     * case any account can execute it.
     *
     * Avoid scheduling multiple revoke delay changes for the same action at the same time, as this makes it harder to
     * reason about the state of the system. If there is already a scheduled revoke delay change and there is a desire
     * to change the future grant delay to some other value, cancel the first scheduled change and schedule a new one.
     *
     * Only root can call this function, but other accounts may be granted permission to cancel the scheduled execution
     * (including global cancelers).
     */
    function scheduleRevokeDelayChange(
        bytes32 actionId,
        uint256 newDelay,
        address[] memory executors
    ) external returns (uint256);

    /**
     * @notice Schedules an arbitrary execution of `data` in target `where`. Returns a scheduledExecutionId that can be
     * used to call `execute`, `cancel`, and associated getters such as `getScheduledExecution`.
     *
     * If `executors` is an empty array, then any account in the network will be able to initiate the scheduled
     * execution. If not, only accounts in the `executors` array will be able to call `execute`. It is not possible to
     * change this after scheduling: the list of executors is immutable, and cannot be changed by any account (including
     * root).
     *
     * The caller of the `schedule` function is automatically made a canceler for the scheduled execution, meaning they
     * can call the `cancel` function for it. Other accounts, such as root, may also have or be granted permission to
     * cancel any scheduled execution.
     *
     * This is the only way to execute actions in external contracts that have a delay associated with them. Calling
     * said functions directly will cause `canPerform` to return false, even if the caller has permission. An account
     * that has permission over an action with a delay cannot call it directly, and must instead schedule a delayed
     * execution by calling this function.
     */
    function schedule(address where, bytes memory data, address[] memory executors) external returns (uint256);

    /**
     * @notice Grants a permission to a single `account` at 'where' address.
     * @dev This function can only be used for actions that have no grant delay. For those that do, use
     * `scheduleGrantPermission` instead.
     */
    function grantPermission(bytes32 actionId, address account, address where) external;

    /**
     * @notice Schedules a grant permission to `account` for action `actionId` in target `where`.
     * @dev See `schedule` comments.
     */
    function scheduleGrantPermission(
        bytes32 actionId,
        address account,
        address where,
        address[] memory executors
    ) external returns (uint256);

    /**
     * @notice Revokes a permission from a single `account` at `where` address.
     * @dev This function can only be used for actions that have no revoke delay. For those that do, use
     * `scheduleRevokePermission` instead.
     */
    function revokePermission(bytes32 actionId, address account, address where) external;

    /**
     * @notice Schedules a revoke permission from `account` for action `actionId` in target `where`.
     * @dev See `schedule` comments.
     */
    function scheduleRevokePermission(
        bytes32 actionId,
        address account,
        address where,
        address[] memory executors
    ) external returns (uint256);

    /**
     * @notice Revokes a permission from the caller for `actionId` at `where` address
     * @dev Note that the caller can always renounce permissions, even if revoking them would typically be
     * subject to a delay.
     */
    function renouncePermission(bytes32 actionId, address where) external;
}

```


## ./pkg/interfaces/contracts/vault/IPoolLiquidity.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/vault/IPoolLiquidity.sol

/// @notice Interface for custom liquidity operations.
interface IPoolLiquidity {
    /**
     * @notice Add liquidity to the pool with a custom hook.
     * @param router The address (usually a router contract) that initiated a swap operation on the Vault
     * @param maxAmountsInScaled18 Maximum input amounts, sorted in token registration order
     * @param minBptAmountOut Minimum amount of output pool tokens
     * @param balancesScaled18 Current pool balances, sorted in token registration order
     * @param userData Arbitrary data sent with the encoded request
     * @return amountsInScaled18 Input token amounts, sorted in token registration order
     * @return bptAmountOut Calculated pool token amount to receive
     * @return swapFeeAmountsScaled18 The amount of swap fees charged for each token
     * @return returnData Arbitrary data with an encoded response from the pool
     */
    function onAddLiquidityCustom(
        address router,
        uint256[] memory maxAmountsInScaled18,
        uint256 minBptAmountOut,
        uint256[] memory balancesScaled18,
        bytes memory userData
    )
        external
        returns (
            uint256[] memory amountsInScaled18,
            uint256 bptAmountOut,
            uint256[] memory swapFeeAmountsScaled18,
            bytes memory returnData
        );

    /**
     * @notice Remove liquidity from the pool with a custom hook.
     * @param router The address (usually a router contract) that initiated a swap operation on the Vault
     * @param maxBptAmountIn Maximum amount of input pool tokens
     * @param minAmountsOutScaled18 Minimum output amounts, sorted in token registration order
     * @param balancesScaled18 Current pool balances, sorted in token registration order
     * @param userData Arbitrary data sent with the encoded request
     * @return bptAmountIn Calculated pool token amount to burn
     * @return amountsOutScaled18 Amount of tokens to receive, sorted in token registration order
     * @return swapFeeAmountsScaled18 The amount of swap fees charged for each token
     * @return returnData Arbitrary data with an encoded response from the pool
     */
    function onRemoveLiquidityCustom(
        address router,
        uint256 maxBptAmountIn,
        uint256[] memory minAmountsOutScaled18,
        uint256[] memory balancesScaled18,
        bytes memory userData
    )
        external
        returns (
            uint256 bptAmountIn,
            uint256[] memory amountsOutScaled18,
            uint256[] memory swapFeeAmountsScaled18,
            bytes memory returnData
        );
}

```


## ./pkg/interfaces/contracts/vault/ISenderGuard.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/vault/ISenderGuard.sol

/// @notice Interface for functions shared across all trusted routers.
interface ISenderGuard {
    /// @notice Incoming ETH transfer from an address that is not WETH.
    error EthTransfer();

    /// @notice The swap transaction was not validated before the specified deadline timestamp.
    error SwapDeadline();

    /**
     * @notice Get the first sender which initialized the call to Router.
     * @return sender The address of the sender
     */
    function getSender() external view returns (address sender);
}

```


## ./pkg/interfaces/contracts/vault/IERC20MultiTokenErrors.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/vault/IERC20MultiTokenErrors.sol

interface IERC20MultiTokenErrors {
    /**
     * @notice The total supply of a pool token can't be lower than the absolute minimum.
     * @param totalSupply The total supply value that was below the minimum
     */
    error PoolTotalSupplyTooLow(uint256 totalSupply);
}

```


## ./pkg/interfaces/contracts/vault/IWrappedBalancerPoolToken.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/vault/IWrappedBalancerPoolToken.sol

/// @notice Interface for wrapped Balancer pool tokens
interface IWrappedBalancerPoolToken {
    /// @notice The vault is unlocked
    error VaultIsUnlocked();

    /**
     * @notice Mints wrapped BPTs in exchange for locked BPTs
     * @param amount The amount of locked BPTs to exchange for wrapped BPTs
     */
    function mint(uint256 amount) external;

    /**
     * @notice Burns wrapped BPTs to unlock the underlying locked BPTs
     * @param value The amount of wrapped BPTs to burn in order to unlock locked BPTs
     */
    function burn(uint256 value) external;

    /**
     * @notice Burns wrapped BPTs on behalf of an approved account to unlock their locked BPTs
     * @param account The address from which the wrapped BPTs will be burned
     * @param value The amount of wrapped BPTs to burn in order to unlock locked BPTs
     */
    function burnFrom(address account, uint256 value) external;
}

```


## ./pkg/interfaces/contracts/vault/IBasePoolFactory.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/solidity-utils/helpers/IAuthentication.sol

/// @notice Simple interface for permissioned calling of external functions.
interface IAuthentication {
    /// @notice The sender does not have permission to call a function.
    error SenderNotAllowed();

    /**
     * @notice Returns the action identifier associated with the external function described by `selector`.
     * @param selector The 4-byte selector of the permissioned function
     * @return actionId The computed actionId
     */
    function getActionId(bytes4 selector) external view returns (bytes32 actionId);
}

// pkg/interfaces/contracts/vault/IBasePoolFactory.sol

/**
 * @notice Base interface for a Balancer Pool Factory.
 * @dev All pool factories should be derived from `BasePoolFactory` to enable common behavior for all pool types
 * (e.g., address prediction, tracking deployed pools, and governance-facilitated migration).
 */
interface IBasePoolFactory is IAuthentication {
    /**
     * @notice A pool was deployed.
     * @param pool The address of the new pool
     */
    event PoolCreated(address indexed pool);

    /// @notice The factory was disabled by governance.
    event FactoryDisabled();

    /// @notice Attempted pool creation after the factory was disabled.
    error Disabled();

    /// @notice A pool index is beyond the current bounds of the array.
    error IndexOutOfBounds();

    /**
     * @notice Check whether a pool was deployed by this factory.
     * @param pool The pool to check
     * @return success True if `pool` was created by this factory
     */
    function isPoolFromFactory(address pool) external view returns (bool success);

    /**
     * @notice Return the total number of pools deployed by this factory.
     * @dev This can then be used to "paginate" calls to `getPools` to control gas costs.
     * @return poolCount The number of pools deployed by this factory
     */
    function getPoolCount() external view returns (uint256 poolCount);

    /**
     * @notice Return a subset of the list of pools deployed by this factory.
     * @dev `start` must be a valid index, but if `count` exceeds the total length, it will not revert, but simply
     * stop at the end and return fewer results than requested.
     *
     * @param start The index of the first pool to return
     * @param count The maximum number of pools to return
     * @return pools The list of pools deployed by this factory, starting at `start` and returning up to `count` pools
     */
    function getPoolsInRange(uint256 start, uint256 count) external view returns (address[] memory pools);

    /**
     * @notice Return the complete list of pools deployed by this factory.
     * @return pools The list of pools deployed by this factory
     */
    function getPools() external view returns (address[] memory pools);

    /**
     * @notice Return the address where a new pool will be deployed, based on the factory address and salt.
     * @param constructorArgs The arguments used to create the pool
     * @param salt The salt used to deploy the pool
     * @return deploymentAddress The predicted address of the pool, given the salt
     */
    function getDeploymentAddress(
        bytes memory constructorArgs,
        bytes32 salt
    ) external view returns (address deploymentAddress);

    /**
     * @notice Check whether this factory has been disabled by governance.
     * @return success True if this factory was disabled
     */
    function isDisabled() external view returns (bool success);

    /**
     * @notice Disable the factory, preventing the creation of more pools.
     * @dev Existing pools are unaffected. Once a factory is disabled, it cannot be re-enabled.
     */
    function disable() external;
}

```


## ./pkg/interfaces/contracts/vault/ICompositeLiquidityRouter.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/vault/ICompositeLiquidityRouterErrors.sol

/// @notice Errors are declared inside an interface (namespace) to improve DX with Typechain.
interface ICompositeLiquidityRouterErrors {
    /**
     * @notice The actual result of the liquidity removal operation does not match the expected set of tokens.
     * @param actualTokensOut The set of tokens derived from pool traversal
     * @param expectedTokensOut The set of tokens supplied by the user
     */
    error WrongTokensOut(address[] actualTokensOut, address[] expectedTokensOut);

    /**
     * @notice The `tokensIn` array contains a duplicate token.
     * @dev Note that it's technically possible to have duplicate tokens with 0 amounts, as those are ignored.
     * @param duplicateToken The address of the duplicate token
     */
    error DuplicateTokenIn(address duplicateToken);
}

// pkg/interfaces/contracts/vault/ICompositeLiquidityRouter.sol

/**
 * @notice The composite liquidity router supports add/remove liquidity operations on ERC4626 and nested pools.
 * @dev This contract allow interacting with ERC4626 Pools (which contain wrapped ERC4626 tokens) using only underlying
 * standard tokens. For instance, with `addLiquidityUnbalancedToERC4626Pool` it is possible to add liquidity to an
 * ERC4626 Pool with [waDAI, waUSDC], using only DAI, only USDC, or an arbitrary amount of both. If the ERC4626 buffers
 * in the Vault have liquidity, these will be used to avoid wrapping/unwrapping through the wrapped token interface,
 * saving gas.
 *
 * For instance, adding only DAI to the pool above (and assuming a waDAI buffer with enough liquidity), would pull in
 * the DAI from the address, swap it for waDAI in the internal Vault buffer, and deposit the waDAI into the ERC4626 pool:
 * 1) without having to do any expensive ERC4626 wrapping operations; and
 * 2) without requiring the address to construct a batch operation containing the buffer swap.
 */
interface ICompositeLiquidityRouter is ICompositeLiquidityRouterErrors {
    /***************************************************************************
                                   ERC4626 Pools
    ***************************************************************************/

    /**
     * @notice Add arbitrary amounts of tokens to an ERC4626 pool through the buffer.
     * @dev An "ERC4626 pool" contains IERC4626 yield-bearing tokens (e.g., waDAI). Ensure that any buffers associated
     * with the wrapped tokens in the ERC4626 pool have been initialized before initializing or adding liquidity to
     * the "parent" pool, and also make sure limits are set properly.
     *
     * @param pool Address of the liquidity pool
     * @param wrapUnderlying Flags indicating whether the corresponding token should be wrapped or used as an ERC20
     * @param exactAmountsIn Exact amounts of underlying/wrapped tokens in, sorted in token registration order
     * @param minBptAmountOut Minimum amount of pool tokens to be received
     * @param wethIsEth If true, incoming ETH will be wrapped to WETH and outgoing WETH will be unwrapped to ETH
     * @param userData Additional (optional) data required for adding liquidity
     * @return bptAmountOut Actual amount of pool tokens received
     */
    function addLiquidityUnbalancedToERC4626Pool(
        address pool,
        bool[] memory wrapUnderlying,
        uint256[] memory exactAmountsIn,
        uint256 minBptAmountOut,
        bool wethIsEth,
        bytes memory userData
    ) external payable returns (uint256 bptAmountOut);

    /**
     * @notice Queries an `addLiquidityUnbalancedToERC4626Pool` operation without actually executing it.
     * @dev An "ERC4626 pool" contains IERC4626 yield-bearing tokens (e.g., waDAI).
     * @param pool Address of the liquidity pool
     * @param wrapUnderlying Flags indicating whether the corresponding token should be wrapped or used as an ERC20
     * @param exactAmountsIn Exact amounts of underlying/wrapped tokens in, sorted in token registration order
     * @param sender The sender passed to the operation. It can influence results (e.g., with user-dependent hooks)
     * @param userData Additional (optional) data required for the query
     * @return bptAmountOut Expected amount of pool tokens to receive
     */
    function queryAddLiquidityUnbalancedToERC4626Pool(
        address pool,
        bool[] memory wrapUnderlying,
        uint256[] memory exactAmountsIn,
        address sender,
        bytes memory userData
    ) external returns (uint256 bptAmountOut);

    /**
     * @notice Add proportional amounts of tokens to an ERC4626 pool through the buffer.
     * @dev An "ERC4626 pool" contains IERC4626 yield-bearing tokens (e.g., waDAI). Ensure that any buffers associated
     * with the wrapped tokens in the ERC4626 pool have been initialized before initializing or adding liquidity to
     * the "parent" pool, and also make sure limits are set properly.
     *
     * @param pool Address of the liquidity pool
     * @param wrapUnderlying Flags indicating whether the corresponding token should be wrapped or used as an ERC20
     * @param maxAmountsIn Maximum amounts of underlying/wrapped tokens in, sorted in token registration order
     * @param exactBptAmountOut Exact amount of pool tokens to be received
     * @param wethIsEth If true, incoming ETH will be wrapped to WETH and outgoing WETH will be unwrapped to ETH
     * @param userData Additional (optional) data required for adding liquidity
     * @return amountsIn Actual amounts of tokens added to the pool
     */
    function addLiquidityProportionalToERC4626Pool(
        address pool,
        bool[] memory wrapUnderlying,
        uint256[] memory maxAmountsIn,
        uint256 exactBptAmountOut,
        bool wethIsEth,
        bytes memory userData
    ) external payable returns (uint256[] memory amountsIn);

    /**
     * @notice Queries an `addLiquidityProportionalToERC4626Pool` operation without actually executing it.
     * @dev An "ERC4626 pool" contains IERC4626 yield-bearing tokens (e.g., waDAI).
     * @param pool Address of the liquidity pool
     * @param wrapUnderlying Flags indicating whether the corresponding token should be wrapped or used as an ERC20
     * @param exactBptAmountOut Exact amount of pool tokens to be received
     * @param sender The sender passed to the operation. It can influence results (e.g., with user-dependent hooks)
     * @param userData Additional (optional) data required for the query
     * @return amountsIn Expected amounts of tokens added to the pool
     */
    function queryAddLiquidityProportionalToERC4626Pool(
        address pool,
        bool[] memory wrapUnderlying,
        uint256 exactBptAmountOut,
        address sender,
        bytes memory userData
    ) external returns (uint256[] memory amountsIn);

    /**
     * @notice Remove proportional amounts of tokens from an ERC4626 pool, burning an exact pool token amount.
     * @dev An "ERC4626 pool" contains IERC4626 yield-bearing tokens (e.g., waDAI).
     * @param pool Address of the liquidity pool
     * @param unwrapWrapped Flags indicating whether the corresponding token should be unwrapped or used as an ERC20
     * @param exactBptAmountIn Exact amount of pool tokens provided
     * @param minAmountsOut Minimum amounts of each token, sorted in token registration order
     * @param wethIsEth If true, incoming ETH will be wrapped to WETH and outgoing WETH will be unwrapped to ETH
     * @param userData Additional (optional) data required for removing liquidity
     * @return amountsOut Actual amounts of tokens received
     */
    function removeLiquidityProportionalFromERC4626Pool(
        address pool,
        bool[] memory unwrapWrapped,
        uint256 exactBptAmountIn,
        uint256[] memory minAmountsOut,
        bool wethIsEth,
        bytes memory userData
    ) external payable returns (uint256[] memory amountsOut);

    /**
     * @notice Queries a `removeLiquidityProportionalFromERC4626Pool` operation without actually executing it.
     * @dev An "ERC4626 pool" contains IERC4626 yield-bearing tokens (e.g., waDAI).
     * @param pool Address of the liquidity pool
     * @param unwrapWrapped Flags indicating whether the corresponding token should be unwrapped or used as an ERC20
     * @param exactBptAmountIn Exact amount of pool tokens provided for the query
     * @param sender The sender passed to the operation. It can influence results (e.g., with user-dependent hooks)
     * @param userData Additional (optional) data required for the query
     * @return amountsOut Expected amounts of tokens to receive
     */
    function queryRemoveLiquidityProportionalFromERC4626Pool(
        address pool,
        bool[] memory unwrapWrapped,
        uint256 exactBptAmountIn,
        address sender,
        bytes memory userData
    ) external returns (uint256[] memory amountsOut);

    /***************************************************************************
                                   Nested pools
    ***************************************************************************/

    /**
     * @notice Adds liquidity unbalanced to a nested pool.
     * @dev A nested pool is one in which one or more tokens are BPTs from another pool (child pool). Since there are
     * multiple pools involved, the token order is not well-defined, and must be specified by the caller. If the parent
     * or nested pools contain ERC4626 tokens that appear in the `tokensToWrap` list, they will be wrapped and their
     * underlying tokens pulled as input, and expected to appear in `tokensIn`. Otherwise, they will be treated as
     * regular tokens.
     *
     * NB: Pools with "overlapping" tokens (i.e., both the parent and a child pool contain one or more of the tokens in
     * `tokensIn`), are not supported! The gas cost to explicitly detect this rare edge case would be prohibitive, so
     * behavior in this case is undefined.
     *
     * @param parentPool The address of the parent pool (which contains BPTs of other pools)
     * @param tokensIn An array with all tokens from the child pools, and all non-BPT parent tokens, in arbitrary order
     * @param exactAmountsIn An array with the amountIn of each token, sorted in the same order as tokensIn
     * @param tokensToWrap A list of ERC4626 tokens which should be wrapped if encountered during pool traversal
     * @param minBptAmountOut Expected minimum amount of parent pool tokens to receive
     * @param wethIsEth If true, incoming ETH will be wrapped to WETH and outgoing WETH will be unwrapped to ETH
     * @param userData Additional (optional) data required for the operation
     * @return bptAmountOut The actual amount of parent pool tokens received
     */
    function addLiquidityUnbalancedNestedPool(
        address parentPool,
        address[] memory tokensIn,
        uint256[] memory exactAmountsIn,
        address[] memory tokensToWrap,
        uint256 minBptAmountOut,
        bool wethIsEth,
        bytes memory userData
    ) external payable returns (uint256 bptAmountOut);

    /**
     * @notice Queries an `addLiquidityUnbalancedNestedPool` operation without actually executing it.
     * @param parentPool The address of the parent pool (which contains BPTs of other pools)
     * @param tokensIn An array with all tokens from the child pools, and all non-BPT parent tokens, in arbitrary order
     * @param exactAmountsIn An array with the amountIn of each token, sorted in the same order as tokensIn
     * @param tokensToWrap A list of ERC4626 tokens which should be wrapped if encountered during pool traversal
     * @param sender The sender passed to the operation. It can influence results (e.g., with user-dependent hooks)
     * @param userData Additional (optional) data required for the operation
     * @return bptAmountOut The actual amount of parent pool tokens received
     */
    function queryAddLiquidityUnbalancedNestedPool(
        address parentPool,
        address[] memory tokensIn,
        uint256[] memory exactAmountsIn,
        address[] memory tokensToWrap,
        address sender,
        bytes memory userData
    ) external returns (uint256 bptAmountOut);

    /**
     * @notice Removes liquidity from a nested pool.
     * @dev A nested pool is one in which one or more tokens are BPTs from another pool (child pool). Since there are
     * multiple pools involved, the token order is not well-defined, and must be specified by the caller. If the parent
     * or nested pools contain ERC4626 tokens that appear in the `tokensToUnwrap` list, they will be unwrapped and
     * their underlying tokens sent to the output. Otherwise, they will be treated as regular tokens.
     *
     * @param parentPool The address of the parent pool (which contains BPTs of other pools)
     * @param exactBptAmountIn The exact amount of `parentPool` tokens provided
     * @param tokensOut An array with all tokens from the child pools, and all non-BPT parent tokens, in arbitrary order
     * @param minAmountsOut An array with the minimum amountOut of each token, sorted in the same order as tokensOut
     * @param tokensToUnwrap A list of ERC4626 tokens which should be unwrapped if encountered during pool traversal
     * @param wethIsEth If true, incoming ETH will be wrapped to WETH and outgoing WETH will be unwrapped to ETH
     * @param userData Additional (optional) data required for the operation
     * @return amountsOut An array with the actual amountOut of each token, sorted in the same order as tokensOut
     */
    function removeLiquidityProportionalNestedPool(
        address parentPool,
        uint256 exactBptAmountIn,
        address[] memory tokensOut,
        uint256[] memory minAmountsOut,
        address[] memory tokensToUnwrap,
        bool wethIsEth,
        bytes memory userData
    ) external payable returns (uint256[] memory amountsOut);

    /**
     * @notice Queries an `removeLiquidityProportionalNestedPool` operation without actually executing it.
     * @param parentPool The address of the parent pool (which contains BPTs of other pools)
     * @param exactBptAmountIn The exact amount of `parentPool` tokens provided
     * @param tokensOut An array with all tokens from the child pools, and all non-BPT parent tokens, in arbitrary order
     * @param tokensToUnwrap A list of ERC4626 tokens which should be unwrapped if encountered during pool traversal
     * @param sender The sender passed to the operation. It can influence results (e.g., with user-dependent hooks)
     * @param userData Additional (optional) data required for the operation
     * @return amountsOut An array with the expected amountOut of each token, sorted in the same order as tokensOut
     */
    function queryRemoveLiquidityProportionalNestedPool(
        address parentPool,
        uint256 exactBptAmountIn,
        address[] memory tokensOut,
        address[] memory tokensToUnwrap,
        address sender,
        bytes memory userData
    ) external returns (uint256[] memory amountsOut);
}

```


## ./pkg/interfaces/contracts/vault/IUnbalancedLiquidityInvariantRatioBounds.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/vault/IUnbalancedLiquidityInvariantRatioBounds.sol

/**
 * @notice Return the minimum/maximum invariant ratios allowed during an unbalanced liquidity operation.
 * @dev The Vault does not enforce any "baseline" bounds on invariant ratios, since such bounds are highly specific
 * and dependent on the math of each pool type. Instead, the Vault reads invariant ratio bounds from the pools.
 * `IBasePool` implements this interface to ensure that new pool developers think about and set these bounds according
 * to their pool type's math.
 *
 * For instance, Balancer Weighted Pool math involves exponentiation (the `pow` function), which uses natural
 * logarithms and a discrete Taylor series expansion to compute x^y values for the 18-decimal floating point numbers
 * used in all Vault computations. See `LogExpMath` and `WeightedMath` for a derivation of the bounds for these pools.
 */
interface IUnbalancedLiquidityInvariantRatioBounds {
    /// @return minimumInvariantRatio The minimum invariant ratio for a pool during unbalanced remove liquidity
    function getMinimumInvariantRatio() external view returns (uint256 minimumInvariantRatio);

    /// @return maximumInvariantRatio The maximum invariant ratio for a pool during unbalanced add liquidity
    function getMaximumInvariantRatio() external view returns (uint256 maximumInvariantRatio);
}

```


## ./pkg/interfaces/contracts/vault/ISwapFeePercentageBounds.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/vault/ISwapFeePercentageBounds.sol

/**
 * @notice Return the minimum/maximum swap fee percentages for a pool.
 * @dev The Vault does not enforce bounds on swap fee percentages; `IBasePool` implements this interface to ensure
 * that new pool developers think about and set these bounds according to their specific pool type.
 *
 * A minimum swap fee might be necessary to ensure mathematical soundness (e.g., Weighted Pools, which use the power
 * function in the invariant). A maximum swap fee is general protection for addresses. With no limits at the Vault level,
 * a pool could specify a near 100% swap fee, effectively disabling trading. Though there are some use cases, such as
 * LVR/MEV strategies, where a very high fee makes sense.
 *
 * Note that the Vault does ensure that dynamic and aggregate fees are less than 100% to prevent attempting to allocate
 * more fees than were collected by the operation. The true `MAX_FEE_PERCENTAGE` is defined in VaultTypes.sol, and is
 * the highest value below 100% that satisfies the precision requirements.
 */
interface ISwapFeePercentageBounds {
    /// @return minimumSwapFeePercentage The minimum swap fee percentage for a pool
    function getMinimumSwapFeePercentage() external view returns (uint256 minimumSwapFeePercentage);

    /// @return maximumSwapFeePercentage The maximum swap fee percentage for a pool
    function getMaximumSwapFeePercentage() external view returns (uint256 maximumSwapFeePercentage);
}

```


## ./pkg/interfaces/contracts/vault/IAuthorizer.sol

```solidity
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

// pkg/interfaces/contracts/vault/IAuthorizer.sol

/// @notice Interface to the Vault's permission system.
interface IAuthorizer {
    /**
     * @notice Returns true if `account` can perform the action described by `actionId` in the contract `where`.
     * @param actionId Identifier for the action to be performed
     * @param account Account trying to perform the action
     * @param where Target contract for the action
     * @return success True if the action is permitted
     */
    function canPerform(bytes32 actionId, address account, address where) external view returns (bool success);
}

```


---

## Summary

- **Total files processed**: 207
- **Successfully flattened**: 46
- **Failed to flatten**: 161
- **Generated on**: Wed 29 Oct 2025 22:30:14 HKT

