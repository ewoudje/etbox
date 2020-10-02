package com.ewoudje.asmetbox.parser.asm8051;

import com.ewoudje.asmetbox.parser.ContextCalculator;
import com.ewoudje.asmetbox.parser.MissingContextException;
import com.ewoudje.asmetbox.parser.Value;

/**
 * User: ewoudje
 * Date: 1/10/20
 * Time: 14:49
 */
public class BaseValue implements Value<Asm8051GenListener> {
	private int calculated = -1;
	private ContextCalculator<Asm8051GenListener> calculator;
	private Asm8051GenListener context;

	public BaseValue() {

	}

	public BaseValue(int v) {
		calculated = v;
	}

	public BaseValue(ContextCalculator<Asm8051GenListener> calc) {
		this.calculator = calc;
	}

	public BaseValue(Asm8051GenListener context, ContextCalculator<Asm8051GenListener> calc) {
		this.context = context;
		this.calculator = calc;
	}

	public void calculate(Asm8051GenListener context) throws MissingContextException {
		calculated = calculator.calculate(context);
	}

	public void setCalculator(ContextCalculator<Asm8051GenListener> calculator) {
		this.calculator = calculator;
	}

	public void setContext(Asm8051GenListener context) {
		this.context = context;
	}

	@Override
	public int getAsInt() {
		if (calculated == -1) {
			try {
				calculate(context);
			} catch (MissingContextException e) {
				e.printStackTrace();
				throw new UnsupportedOperationException();
			}
		}
		return calculated;
	}
}
