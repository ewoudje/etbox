import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';

import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind
} from 'vscode-languageclient';

let client: LanguageClient;

export function activate(context: ExtensionContext) {

    // The server is implemented in node
    let serverModule = context.asAbsolutePath(path.join('etbox.jar'));

    // If the extension is launched in debug mode then the debug server options are used
    // Otherwise the run options are used
    let serverOptions: ServerOptions = {
        run: { command: "java", args: ["-jar", serverModule, "server", "pipe", "-"] },
        debug: { command: "java", args: ["-agentlib:jdwp=transport=dt_socket,server=y,suspend=n,address=5005",
                "-jar", serverModule, "server", "pipe", "-"]}
    };

    // Options to control the language client
    let clientOptions: LanguageClientOptions = {
        // Register the server for plain text documents
        documentSelector: [{ scheme: 'file', language: 'plaintext' }],
        synchronize: {
            // Notify the server about file changes to '.asm' files contained in the workspace
            fileEvents: workspace.createFileSystemWatcher('**/*.asm')
        }
    };

    // Create the language client and start the client.
    client = new LanguageClient(
        'etbox',
        'Ewoud\'s Language Toolbox',
        serverOptions,
        clientOptions
    );

    // Start the client. This will also launch the server
    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}