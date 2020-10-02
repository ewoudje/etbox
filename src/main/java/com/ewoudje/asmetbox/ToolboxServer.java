package com.ewoudje.asmetbox;

import com.ewoudje.asmetbox.lsp.LanguageServerBase;
import org.eclipse.lsp4j.jsonrpc.Launcher;
import org.eclipse.lsp4j.launch.LSPLauncher;
import org.eclipse.lsp4j.services.LanguageClient;

/**
 * User: ewoudje
 * Date: 30/09/20
 * Time: 10:45
 */
public class ToolboxServer {

	private LanguageServerBase langServer;
	private InOutStreamSupplier supplier;
	private Launcher<LanguageClient> launcher;

	public ToolboxServer(InOutStreamSupplier supplier) {
		this.supplier = supplier;
	}

	public void init() {
		langServer = new LanguageServerBase();

		launcher = LSPLauncher.createServerLauncher(langServer, supplier.getInput(), supplier.getOutput());
	}

	public void run() {

		launcher.startListening();
	}
}
