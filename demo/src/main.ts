// import Monaco Language Client components
import { EditorApp, type EditorAppConfig } from 'monaco-languageclient/editorApp';
import { configureDefaultWorkerFactory } from 'monaco-languageclient/workerFactory';
import { MonacoVscodeApiWrapper, type MonacoVscodeApiConfig } from 'monaco-languageclient/vscodeApiWrapper';
import { LanguageClientWrapper, type LanguageClientConfig } from 'monaco-languageclient/lcwrapper';
// VSCode API for file system operations
import * as vscode from 'vscode';
import { LogLevel } from '@codingame/monaco-vscode-api';
import {
  RegisteredFileSystemProvider,
  RegisteredMemoryFile,
  registerFileSystemOverlay
} from '@codingame/monaco-vscode-files-service-override';

import {start_language_server} from './language-server-launcher.ts';
import { mumps_language_id, mumpsExampleConntent, mumpsExtension } from './mumps-extention.ts';


async function createMumpsEditor() {

  const lsp_connection =  start_language_server();
  
  // Set up an in-memory file system (won't persist on reload)
  const fileUri = vscode.Uri.file('/workspace/example.m');
  const fileSystemProvider = new RegisteredFileSystemProvider(false);
  fileSystemProvider.registerFile(new RegisteredMemoryFile(fileUri, mumpsExampleConntent));
  registerFileSystemOverlay(1, fileSystemProvider);

  // Monaco VSCode API configuration
  const vscodeApiConfig: MonacoVscodeApiConfig = {
    $type: 'extended',
    viewsConfig: {
      $type: 'EditorService',
      htmlContainer: document.getElementById('monaco-editor-root')!
    },
    logLevel: LogLevel.Debug,
    extensions: [{ config: mumpsExtension }],
    advanced: {
      enforceSemanticHighlighting: true
    },
    userConfiguration: {
      json: JSON.stringify({
        'workbench.colorTheme': 'Default Dark Modern',
        'editor.guides.bracketPairsHorizontal': 'active',
        'editor.lightbulb.enabled': 'On',
        'editor.wordBasedSuggestions': 'off',
        'editor.experimental.asyncTokenization': true
      })
    },
    monacoWorkerFactory: configureDefaultWorkerFactory
  };

  // Language client configuration
  const languageClientConfig: LanguageClientConfig = {
    languageId: mumps_language_id,
    connection: lsp_connection,
    clientOptions: {
      documentSelector: [mumps_language_id],
      workspaceFolder: {
        index: 0,
        name: 'workspace',
        uri: vscode.Uri.file('/workspace')
      }
    }
  };

  // editor app / Monaco-editor configuration
  const editorAppConfig: EditorAppConfig = {
    codeResources: {
      modified: {
        text: mumpsExampleConntent,
        uri: fileUri.path
      }
    }
  };

  // Create the Monaco-VScode API Wrapper
  const apiWrapper = new MonacoVscodeApiWrapper(vscodeApiConfig);
  await apiWrapper.start();

  // create language client wrapper & app
  const lcWrapper = new LanguageClientWrapper(languageClientConfig);
  const editorApp = new EditorApp(editorAppConfig);

  // start editor app first, then language client
  await editorApp.start(document.getElementById('monaco-editor-root')!);
  await lcWrapper.start();

  console.log('JSON editor with language client is ready!');
}
createMumpsEditor().catch(console.error);
