package com.ewoudje.asmetbox.parser.asm8051;

import com.ewoudje.asmetbox.parser.RuntimeValue;
import com.ewoudje.asmetbox.parser.Value;

/**
 * User: ewoudje
 * Date: 1/10/20
 * Time: 16:06
 */
public class PointerValue implements RuntimeValue {

	private RuntimeValue value;

	public PointerValue(RuntimeValue value) {
		this.value = value;
	}

	@Override
	public int getRuntimeValue() {
		return value.getRuntimeValue();
	}

	@Override
	public int getType() {
		return 3;
	}
}
