import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
    const selector = { language: 'scrust', scheme: 'file' };
    const provider = new ScrustSemanticTokensProvider();
    const legend = new vscode.SemanticTokensLegend(tokenTypes, tokenModifiers);

    context.subscriptions.push(
        vscode.languages.registerDocumentSemanticTokensProvider(selector, provider, legend)
    );
}

export function deactivate() {}

const tokenTypes = ['function'];
const tokenModifiers = ['declaration'];

class ScrustSemanticTokensProvider implements vscode.DocumentSemanticTokensProvider {
    async provideDocumentSemanticTokens(document: vscode.TextDocument, token: vscode.CancellationToken): Promise<vscode.SemanticTokens> {
        const builder = new vscode.SemanticTokensBuilder();
        const text = document.getText();
        const procNames = new Set<string>();

        // First pass: Find all proc definitions
        const procDefRegex = /\bproc\s+([a-zA-Z_][a-zA-Z0-9_]*)/g;
        let match;
        while ((match = procDefRegex.exec(text)) !== null) {
            if (match[1]) {
                procNames.add(match[1]);
            }
        }

        // Second pass: Find all calls to these procs
        const callRegex = /\b([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/g;
        while ((match = callRegex.exec(text)) !== null) {
            const name = match[1];
            const index = match.index;

            if (procNames.has(name)) {
                // It's a call to a known proc!
                const startPos = document.positionAt(index);
                builder.push(
                    startPos.line,
                    startPos.character,
                    name.length,
                    0, // index in tokenTypes ('function')
                    0  // index in tokenModifiers
                );
            }
        }

        return builder.build();
    }
}
