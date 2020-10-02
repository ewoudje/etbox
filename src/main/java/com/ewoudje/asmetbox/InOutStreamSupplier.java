package com.ewoudje.asmetbox;

import java.io.InputStream;
import java.io.OutputStream;

/**
 * User: ewoudje
 * Date: 30/09/20
 * Time: 12:43
 */
public interface InOutStreamSupplier {

	InputStream getInput();

	OutputStream getOutput();

}
