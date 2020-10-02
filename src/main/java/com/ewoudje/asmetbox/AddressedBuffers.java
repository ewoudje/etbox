package com.ewoudje.asmetbox;

import java.nio.ByteBuffer;
import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;

/**
 * User: ewoudje
 * Date: 1/10/20
 * Time: 10:16
 */
public class AddressedBuffers {
	private HashMap<Integer, ByteBuffer> buffers = new HashMap<>();

	public void addBuffer(int address, ByteBuffer buffer) {
		buffers.put(address, buffer);
	}

	public Iterator<Map.Entry<Integer, ByteBuffer>> iterator() {
		return buffers.entrySet().iterator();
	}
}
