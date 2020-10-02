package com.ewoudje.asmetbox;

import java.io.BufferedWriter;
import java.io.IOException;
import java.io.Writer;
import java.nio.ByteBuffer;

/**
 * User: ewoudje
 * Date: 1/10/20
 * Time: 10:01
 */
public class Hexifier {

	private final AddressedBuffers buffers;

	public Hexifier(AddressedBuffers buffers) {
		this.buffers = buffers;
	}

	public void writeHex(Writer writer) throws IOException {
		BufferedWriter fos = new BufferedWriter(writer);
		//TODO hexify
		fos.flush();
		fos.close();
	}

}
