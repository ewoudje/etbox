package com.ewoudje.asmetbox.parser.asm8051;

import com.ewoudje.asm8051.Asm8051Lexer;
import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.Token;
import org.antlr.v4.runtime.TokenSource;
import org.antlr.v4.runtime.UnbufferedTokenStream;

import java.io.FileInputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.Stack;

/**
 * User: ewoudje
 * Date: 1/10/20
 * Time: 11:28
 */
public class PreProcessorTokenStream extends UnbufferedTokenStream {

	private Stack<TokenSource> sources = new Stack<>();

	public PreProcessorTokenStream(TokenSource tokenSource) {
		super(tokenSource);
	}

	@Override
	protected void add(Token t) {
		switch(t.getType()) {
			case Asm8051Lexer.INCLUDE_PREPROCESSOR:
				sources.push(tokenSource);
				String text = t.getText();
				int end = text.lastIndexOf("\"");
				int start = text.indexOf("\"");
				try {
					tokenSource = new Asm8051Lexer(CharStreams.fromStream(new FileInputStream(text.substring(start + 1, end)), StandardCharsets.UTF_8));
				} catch (IOException e) {
					e.printStackTrace();
				}
				return;
			case -1:
				if (!sources.empty()) {
					tokenSource = sources.pop();
					return;
				}
			default:
				super.add(t);
		}
	}
}
