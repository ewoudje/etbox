package com.ewoudje.asmetbox.lsp;

import org.eclipse.lsp4j.DidChangeConfigurationParams;
import org.eclipse.lsp4j.DidChangeWatchedFilesParams;
import org.eclipse.lsp4j.services.WorkspaceService;

/**
 * User: ewoudje
 * Date: 30/09/20
 * Time: 14:07
 */
public class WorkspaceServiceBase implements WorkspaceService {
	@Override
	public void didChangeConfiguration(DidChangeConfigurationParams didChangeConfigurationParams) {

	}

	@Override
	public void didChangeWatchedFiles(DidChangeWatchedFilesParams didChangeWatchedFilesParams) {

	}
}
