import * as vscode from 'vscode';
import type { RegisterLocalProcessExtensionResult } from '@codingame/monaco-vscode-api/extensions';
import { configure } from './config.ts';
import { MonacoVscodeApiWrapper } from 'monaco-languageclient/vscodeApiWrapper';
import { LanguageClientWrapper } from 'monaco-languageclient/lcwrapper';
import { initLocaleLoader } from 'monaco-languageclient/vscodeApiLocales';

export const startDemo = async () => {
  const configResult = await configure(document.getElementById('monaco-editor-root') || document.body);

  const apiWrapper = new MonacoVscodeApiWrapper(configResult.vscodeApiConfig);
  await apiWrapper.start();

  const result = apiWrapper.getExtensionRegisterResult('rsm-demo') as RegisterLocalProcessExtensionResult;
  await result.setAsDefaultApi();

  await Promise.all(configResult.mumpsUris.map((uri) => vscode.workspace.openTextDocument(uri)));
  await vscode.window.showTextDocument(configResult.mumpsUris[0]);

  const lcWrapper = new LanguageClientWrapper(configResult.languageClientConfig);
  await lcWrapper.start();

  console.log('RSM demo started');
};

await initLocaleLoader();

await startDemo();
