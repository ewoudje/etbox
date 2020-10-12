package com.ewoudje.asmetbox.parser.asm8051;

import com.ewoudje.asmetbox.AddressedBuffers;
import com.ewoudje.asmetbox.parser.ContextCalculator;
import com.ewoudje.asmetbox.parser.MissingContextException;
import com.ewoudje.asmetbox.parser.Value;
import jnr.ffi.LibraryLoader;

/**
 * User: ewoudje
 * Date: 1/10/20
 * Time: 09:41
 */
public class Asm8051RustGen implements Asm8051Generator {

	private static final CompilerLib compilerLib = LibraryLoader.create(CompilerLib.class).load("compiler_8051");

	private AddressedBuffers buffers = new AddressedBuffers();
	private int origin;
	private long rustPtr = compilerLib.create_compiler();

	public AddressedBuffers getResult() {
		return buffers;
	}

	@Override
	public void setOrigin(Value<Asm8051GenListener> value) {
		if (value.getAsInt() == origin) return;
		origin = value.getAsInt();
		compilerLib.new_graph(rustPtr,  origin);
	}

	@Override
	public void createInstruction(String instr, Value<Asm8051GenListener> param1, Value<Asm8051GenListener> param2, Value<Asm8051GenListener> param3, int instr_id) {
		if (param1 == null) param1 = new NoValue();
		if (param2 == null) param2 = new NoValue();
		if (param3 == null) param3 = new NoValue();

		compilerLib.instruction(rustPtr,
				param1.getType(), param1.getAsInt(),
				param2.getType(), param2.getAsInt(),
				param3.getType(), param3.getAsInt(),
				instr_id);
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

	public interface CompilerLib {
		void instruction(long compiler, int type1, int value1, int type2, int value2, int type3, int value3, int instr_id);

		long create_compiler();

		void new_graph(long compiler, int origin);
	}
}
