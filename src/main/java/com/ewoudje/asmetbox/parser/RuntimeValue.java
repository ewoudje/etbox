package com.ewoudje.asmetbox.parser;

/**
 * User: ewoudje
 * Date: 1/10/20
 * Time: 14:51
 */
public interface RuntimeValue extends Value {

	@Override
	default void calculate(Object context) throws MissingContextException {
		throw new UnsupportedOperationException();
	}

	@Override
	default void setCalculator(ContextCalculator calculator) {
		throw new UnsupportedOperationException();
	}

	@Override
	default void setContext(Object context) {
		throw new UnsupportedOperationException();
	}

	@Override
	default int getAsInt() {
		return getRuntimeValue();
	}

	int getRuntimeValue();

	@Override
	default int getType() {
		return 1;
	}
}
