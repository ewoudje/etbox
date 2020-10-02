package com.ewoudje.asmetbox;

import com.ewoudje.asmetbox.parser.asm8051.Asm8051Parser;
import com.ewoudje.asmetbox.parser.asm8051.Asm8051RustGen;

import java.io.*;

/**
 * User: ewoudje
 * Date: 30/09/20
 * Time: 10:41
 */
public class Main implements InOutStreamSupplier {

	private ToolboxServer server;
	private boolean stdio = false;
	private File pipe;
	private File file;

	public static void main(String[] args) {
		Main main = new Main();

		for (int i = 0; i < args.length; i++) {
			String arg = args[i];
			if (arg.equals("server")) {
				main.server = new ToolboxServer(main);
			} else if (arg.equals("pipe")) {
				if (++i >= args.length) {
					System.out.println("Invalid use of pipe");
					return;
				}
				String para = args[i];
				if (para.equals("-")) {
					main.stdio = true;
				} else {
					main.pipe = new File(arg);
				}
			} else if (arg.equals("file")) {
				if (++i >= args.length) {
					System.out.println("Invalid use of file");
					return;
				}
				main.file = new File(args[i]);
			} else {
				System.out.println("Unknown arg '" + arg + "'");
				return;
			}
		}

		main.init();

		main.run();
	}

	private void init() {
		if (server != null) {
			server.init();
		} else if (file != null) {
			Asm8051Parser parser;
			try {
				parser = new Asm8051Parser(new FileInputStream(file));
			} catch (IOException e) {
				e.printStackTrace();
				return;
			}

			Asm8051RustGen gen = new Asm8051RustGen();
			parser.generate(gen);
			AddressedBuffers buffers = gen.getResult();
		}
	}

	private void run() {
		if (server != null) {
			server.run();
		}
	}

	@Override
	public InputStream getInput() {
		if (stdio) {
			return System.in;
		} else if (pipe != null) {
			try {
				return new FileInputStream(pipe);
			} catch (FileNotFoundException e) {
				e.printStackTrace();
				return null;
			}
		} else {
			return null;
		}
	}

	@Override
	public OutputStream getOutput() {
		if (stdio) {
			return System.out;
		} else if (pipe != null) {
			try {
				return new FileOutputStream(pipe);
			} catch (FileNotFoundException e) {
				e.printStackTrace();
				return null;
			}
		} else {
			return null;
		}
	}
}
