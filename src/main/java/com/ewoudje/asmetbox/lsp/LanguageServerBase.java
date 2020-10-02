package com.ewoudje.asmetbox.lsp;

import org.eclipse.lsp4j.InitializeParams;
import org.eclipse.lsp4j.InitializeResult;
import org.eclipse.lsp4j.ServerCapabilities;
import org.eclipse.lsp4j.services.LanguageServer;
import org.eclipse.lsp4j.services.TextDocumentService;
import org.eclipse.lsp4j.services.WorkspaceService;

import java.util.concurrent.CompletableFuture;

/**
 * User: ewoudje
 * Date: 30/09/20
 * Time: 10:55
 */
public class LanguageServerBase implements LanguageServer {

	private WorkspaceServiceBase workspaceService = new WorkspaceServiceBase();
	private TextDocumentServiceBase textDocumentService = new TextDocumentServiceBase();

	public CompletableFuture<InitializeResult> initialize(InitializeParams initializeParams) {
		ServerCapabilities cab = new ServerCapabilities();

		return CompletableFuture.completedFuture(new InitializeResult(cab));
	}

	public void exit() {

	}

	public TextDocumentService getTextDocumentService() {
		return textDocumentService;
	}

	public WorkspaceService getWorkspaceService() {
		return workspaceService;
	}

	public CompletableFuture<Object> shutdown() {
		return null;
	}
}
