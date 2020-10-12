package com.ewoudje.asmetbox.parser.asm8051;

import com.ewoudje.asmetbox.parser.RuntimeValue;
import com.ewoudje.asmetbox.parser.Value;

/**
 * User: ewoudje
 * Date: 1/10/20
 * Time: 16:09
 */
public enum RegisterValue implements RuntimeValue {
	ACC,
	DPTR,
	R0, R1, R2, R3,
	R4, R5, R6, R7	
	;

	public static RegisterValue getShortRegister(int n) {
		switch (n) {
			case 0:
				return R0;
			case 1:
				return R1;
			case 2:
				return R2;
			case 3:
				return R3;
			case 4:
				return R4;
			case 5:
				return R5;
			case 6:
				return R6;
			case 7:
				return R7;
			default:
				throw new ArrayIndexOutOfBoundsException();
		}
	}

	@Override
	public int getRuntimeValue() {
		switch (this) {
			case ACC:
				return 1;
			case DPTR:
				return 2;
			case R0:
				return 3;
			case R1:
				return 4;
			case R2:
				return 5;
			case R3:
				return 6;
			case R4:
				return 7;
			case R5:
				return 8;
			case R6:
				return 9;
			case R7:
				return 10;
			default:
				throw new ArrayIndexOutOfBoundsException();
		}
	}

	@Override
	public int getType() {
		return 2;
	}
}
