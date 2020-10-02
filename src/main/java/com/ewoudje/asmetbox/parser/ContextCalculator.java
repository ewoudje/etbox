package com.ewoudje.asmetbox.parser;

/**
 * User: ewoudje
 * Date: 1/10/20
 * Time: 13:54
 */
public interface ContextCalculator<T> {

	int calculate(T context) throws MissingContextException;
}
