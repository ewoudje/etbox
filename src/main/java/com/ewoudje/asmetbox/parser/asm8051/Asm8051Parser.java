package com.ewoudje.asmetbox.parser.asm8051;

import com.ewoudje.asm8051.Asm8051Lexer;
import com.ewoudje.asm8051.Asm8051Listener;
import com.ewoudje.asmetbox.AddressedBuffers;
import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.tree.*;

import java.io.ByteArrayInputStream;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;

/**
 * User: ewoudje
 * Date: 1/10/20
 * Time: 09:37
 */
public class Asm8051Parser {

	private com.ewoudje.asm8051.Asm8051Parser parser;
	private Asm8051Lexer lexer;

	public Asm8051Parser(FileInputStream fileInputStream) throws IOException {
		prepareParser(new Asm8051Lexer(CharStreams.fromStream(fileInputStream, StandardCharsets.UTF_8)));
	}

	public Asm8051Parser(String input) {
		InputStream stream = new ByteArrayInputStream(input.getBytes(StandardCharsets.UTF_8));
		try {
			prepareParser(new Asm8051Lexer(CharStreams.fromStream(stream, StandardCharsets.UTF_8)));
		} catch (IOException e) {
			e.printStackTrace();
		}
	}

	private void prepareParser(Asm8051Lexer lexer) {
		this.lexer = lexer;
		TokenStream tokens = new PreProcessorTokenStream(lexer);
		parser = new com.ewoudje.asm8051.Asm8051Parser(tokens);
	}

	public void generate(Asm8051Generator generator) {
		Asm8051GenListener listener = new Asm8051GenListener(generator);
		ParseTreeWalker.DEFAULT.walk(listener, parser.file());
	}
}
