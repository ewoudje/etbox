package com.ewoudje.asmetbox.parser.asm8051;

import com.ewoudje.asmetbox.parser.Value;

/**
 * User: ewoudje
 * Date: 1/10/20
 * Time: 09:42
 */
public interface Asm8051Generator {
	void setOrigin(Value<Asm8051GenListener> value);

	void createInstruction(String instr, Value<Asm8051GenListener> param1, Value<Asm8051GenListener> param2, Value<Asm8051GenListener> param3, int instr_id);
}
