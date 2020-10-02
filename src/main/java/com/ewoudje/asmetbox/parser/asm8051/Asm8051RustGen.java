package com.ewoudje.asmetbox.parser.asm8051;

import com.ewoudje.asmetbox.AddressedBuffers;
import com.ewoudje.asmetbox.parser.ContextCalculator;
import com.ewoudje.asmetbox.parser.MissingContextException;
import com.ewoudje.asmetbox.parser.RuntimeValue;
import com.ewoudje.asmetbox.parser.Value;

/**
 * User: ewoudje
 * Date: 1/10/20
 * Time: 09:41
 */
public class Asm8051RustGen implements Asm8051Generator {

	private AddressedBuffers buffers = new AddressedBuffers();
	private Value<Asm8051GenListener> origin;
	private long rustPtr = create_compiler();

	private static native long create_compiler();

	private static native void new_origin(long ptr, int pos);

	private static native void instruction(long ptr,
										   int type1, int value1,
										   int type2, int value2,
										   int type3, int value3);

	static {
		System.loadLibrary("compiler_8051");
	}

	public AddressedBuffers getResult() {
		return buffers;
	}

	@Override
	public void setOrigin(Value<Asm8051GenListener> value) {
		origin = value;
	}

	@Override
	public void createInstruction(String instr, Value<Asm8051GenListener> param1, Value<Asm8051GenListener> param2, Value<Asm8051GenListener> param3) {
		if (param1 == null) param1 = new NoValue();
		if (param2 == null) param2 = new NoValue();
		if (param3 == null) param3 = new NoValue();

		instruction(rustPtr,
				param1.getType(), param1.getAsInt(),
				param2.getType(), param2.getAsInt(),
				param3.getType(), param3.getAsInt());
	}


	private static class NoValue implements Value<Asm8051GenListener> {
		@Override
		public void calculate(Asm8051GenListener context) throws MissingContextException {

		}

		@Override
		public void setCalculator(ContextCalculator<Asm8051GenListener> calculator) {

		}

		@Override
		public void setContext(Asm8051GenListener context) {

		}


		@Override
		public int getType() {
			return -1;
		}

		@Override
		public int getAsInt() {
			return -1;
		}
	}
}
