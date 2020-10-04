package com.ewoudje.asmetbox.parser.asm8051;

import com.ewoudje.asmetbox.parser.RuntimeValue;

/**
 * User: ewoudje
 * Date: 4/10/20
 * Time: 12:13
 */
public class SelfValue implements RuntimeValue {

	public static final SelfValue INSTANCE = new SelfValue();

	@Override
	public int getType() {
		return 4;
	}

	@Override
	public int getRuntimeValue() {
		return 0;
	}
}
