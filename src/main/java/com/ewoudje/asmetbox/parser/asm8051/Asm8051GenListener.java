package com.ewoudje.asmetbox.parser.asm8051;

import com.ewoudje.asm8051.Asm8051BaseListener;
import com.ewoudje.asm8051.Asm8051BaseVisitor;
import com.ewoudje.asm8051.Asm8051Parser;
import com.ewoudje.asmetbox.parser.ContextCalculator;
import com.ewoudje.asmetbox.parser.MissingContextException;
import com.ewoudje.asmetbox.parser.RuntimeValue;
import com.ewoudje.asmetbox.parser.Value;
import org.antlr.v4.runtime.ParserRuleContext;

import java.util.HashMap;

/**
 * User: ewoudje
 * Date: 1/10/20
 * Time: 11:47
 */
public class Asm8051GenListener extends Asm8051BaseListener {

	private final Asm8051Generator gen;
	private final ValueCalculatorVisitor vCalc = new ValueCalculatorVisitor();
	private final HashMap<String, BaseValue> labels = new HashMap<>();
	private final Phase2Listener phase2 = new Phase2Listener();

	public Asm8051GenListener(Asm8051Generator gen) {
		this.gen = gen;
	}
	private int current_instr = 0;

	@Override
	public void exitOrg(Asm8051Parser.OrgContext ctx) {
		gen.setOrigin(getValue(ctx.calc_constant()));
	}

	@Override
	public void exitInstruction(Asm8051Parser.InstructionContext ctx) {
		current_instr++;
	}

	private Value<Asm8051GenListener> getValue(ParserRuleContext ctx) {
		if (ctx == null) return null;
		return ctx.accept(vCalc);
	}

	private Value<Asm8051GenListener> getLabel(String label) {
		BaseValue result = labels.get(label);

		if (result == null) {
			result = new BaseValue();
			labels.put(label, result);
		}

		return result;
	}

	private class ValueCalculatorVisitor extends Asm8051BaseVisitor<Value<Asm8051GenListener>> {

		@Override
		public Value<Asm8051GenListener> visitConstant(Asm8051Parser.ConstantContext ctx) {
			return new Value<Asm8051GenListener>() {
				@Override
				public void calculate(Asm8051GenListener context) throws MissingContextException {}

				@Override
				public void setCalculator(ContextCalculator<Asm8051GenListener> calculator) {}

				@Override
				public void setContext(Asm8051GenListener context) {}

				@Override
				public int getAsInt() {
					return visitConstant(ctx).getAsInt();
				}

				@Override
				public int getType() {
					return 5;
				}
			};
		}

		@Override
		public Value<Asm8051GenListener> visitConst_add(Asm8051Parser.Const_addContext ctx) {
			Value v1 = ctx.const_values().accept(this);
			Value v2 = ctx.calc_constant().accept(this);
			return new BaseValue((c) -> v1.getAsInt() + v2.getAsInt());
		}

		@Override
		public Value<Asm8051GenListener> visitConst_sub(Asm8051Parser.Const_subContext ctx) {
			Value v1 = ctx.const_values().accept(this);
			Value v2 = ctx.calc_constant().accept(this);
			return new BaseValue((c) -> v1.getAsInt() - v2.getAsInt());
		}

		@Override
		public Value<Asm8051GenListener> visitBinary(Asm8051Parser.BinaryContext ctx) {
			String binary = ctx.getText();
			binary = binary.substring(0, binary.length() - 1);
			return new BaseValue(Integer.parseInt(binary, 2));
		}

		@Override
		public Value<Asm8051GenListener> visitHexadecimal(Asm8051Parser.HexadecimalContext ctx) {
			String binary = ctx.getText();
			if (binary.endsWith("h"))
				binary = binary.substring(0, binary.length() - 1);
			else
				binary = binary.substring(2);
			return new BaseValue(Integer.parseInt(binary, 16));
		}

		@Override
		public Value<Asm8051GenListener> visitDecimal(Asm8051Parser.DecimalContext ctx) {
			return new BaseValue(Integer.parseInt(ctx.getText()));
		}

		@Override
		public Value<Asm8051GenListener> visitChar_(Asm8051Parser.Char_Context ctx) {
			return new BaseValue(ctx.getText().charAt(1));
		}

		@Override
		public Value<Asm8051GenListener> visitLabelP(Asm8051Parser.LabelPContext ctx) {
			return getLabel(ctx.getText());
		}

		@Override
		public Value<Asm8051GenListener> visitInstr_loc(Asm8051Parser.Instr_locContext ctx) {
			final int instr = current_instr;
			return SelfValue.INSTANCE; //TODO make it an constant not an runtime value!
		}

		@Override
		public Value<Asm8051GenListener> visitLocAt(Asm8051Parser.LocAtContext ctx) {
			return new PointerValue((RuntimeValue) ctx.accept(this));
		}

		@Override
		public Value<Asm8051GenListener> visitShortRegister(Asm8051Parser.ShortRegisterContext ctx) {
			String register = ctx.getText().toLowerCase();
			if (register.equals("a")) {
				return RegisterValue.ACC;
			} else if (register.startsWith("r")) {
				int n = Integer.parseInt(register.substring(1));
				return RegisterValue.getShortRegister(n);
			} else if (register.equals("dptr")) {
				return RegisterValue.DPTR;
			}
			return null;
		}
	}

	private class Phase2Listener extends Asm8051BaseListener {
		private int current_instr2 = 0;

		@Override
		public void exitInstruction(Asm8051Parser.InstructionContext ctx) {
			Value<Asm8051GenListener> param1 = getValue(ctx.parameter(0));
			Value<Asm8051GenListener> param2 = getValue(ctx.parameter(1));
			Value<Asm8051GenListener> param3 = getValue(ctx.parameter(2));

			gen.createInstruction(ctx.instr().getText().toLowerCase(), param1, param2, param3, current_instr2);

			current_instr2++;
		}

	}
}
