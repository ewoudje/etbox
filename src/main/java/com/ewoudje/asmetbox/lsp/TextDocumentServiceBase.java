package com.ewoudje.asmetbox.lsp;

import org.eclipse.lsp4j.DidChangeTextDocumentParams;
import org.eclipse.lsp4j.DidCloseTextDocumentParams;
import org.eclipse.lsp4j.DidOpenTextDocumentParams;
import org.eclipse.lsp4j.DidSaveTextDocumentParams;
import org.eclipse.lsp4j.services.TextDocumentService;

/**
 * User: ewoudje
 * Date: 30/09/20
 * Time: 14:09
 */
public class TextDocumentServiceBase implements TextDocumentService {

	@Override
	public void didOpen(DidOpenTextDocumentParams didOpenTextDocumentParams) {

	}

	@Override
	public void didChange(DidChangeTextDocumentParams didChangeTextDocumentParams) {

	}

	@Override
	public void didClose(DidCloseTextDocumentParams didCloseTextDocumentParams) {

	}

	@Override
	public void didSave(DidSaveTextDocumentParams didSaveTextDocumentParams) {

	}
}
