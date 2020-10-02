package com.ewoudje.asmetbox.parser;

import java.util.function.IntSupplier;

/**
 * User: ewoudje
 * Date: 1/10/20
 * Time: 13:52
 */
public interface Value<T> extends IntSupplier {
	void calculate(T context) throws MissingContextException;

	void setCalculator(ContextCalculator<T> calculator);

	void setContext(T context);

	default int getType() {
		return 0;
	}
}
