import { LogLevel } from '@codingame/monaco-vscode-api';
import getEnvironmentServiceOverride from '@codingame/monaco-vscode-environment-service-override';
import getExplorerServiceOverride from '@codingame/monaco-vscode-explorer-service-override';
import {
  InMemoryFileSystemProvider,
  registerFileSystemOverlay,
  type IFileWriteOptions
} from '@codingame/monaco-vscode-files-service-override';
import getKeybindingsServiceOverride from '@codingame/monaco-vscode-keybindings-service-override';
import type { LanguageClientConfig } from 'monaco-languageclient/lcwrapper';
import getLifecycleServiceOverride from '@codingame/monaco-vscode-lifecycle-service-override';
import getLocalizationServiceOverride from '@codingame/monaco-vscode-localization-service-override';
import getNotificationsServiceOverride from '@codingame/monaco-vscode-notifications-service-override';
import getOutlineServiceOverride from '@codingame/monaco-vscode-outline-service-override';
import getSearchServiceOverride from '@codingame/monaco-vscode-search-service-override';
import getSecretStorageServiceOverride from '@codingame/monaco-vscode-secret-storage-service-override';
import getStorageServiceOverride from '@codingame/monaco-vscode-storage-service-override';
import getBannerServiceOverride from '@codingame/monaco-vscode-view-banner-service-override';
import getStatusBarServiceOverride from '@codingame/monaco-vscode-view-status-bar-service-override';
import getTitleBarServiceOverride from '@codingame/monaco-vscode-view-title-bar-service-override';
import * as vscode from 'vscode';

// required for the workbench's search panel to render search results
import '@codingame/monaco-vscode-search-result-default-extension';

import { createDefaultLocaleConfiguration } from 'monaco-languageclient/vscodeApiLocales';
import { defaultHtmlAugmentationInstructions, defaultViewsInit, type MonacoVscodeApiConfig } from 'monaco-languageclient/vscodeApiWrapper';
import { configureDefaultWorkerFactory } from 'monaco-languageclient/workerFactory';
import { start_language_server } from './language-server-launcher.ts';
import { mumps_language_id, mumpsExampleContent, mumpsExtension } from './mumps-extension.ts';

const createDefaultWorkspaceContent = (workspacePath: string) =>
  JSON.stringify({ folders: [{ path: workspacePath }] }, null, 2);

export type ConfigResult = {
  vscodeApiConfig: MonacoVscodeApiConfig;
  workspaceFileUri: vscode.Uri;
  mumpsUris: vscode.Uri[];
  languageClientConfig: LanguageClientConfig;
};

export const configure = async (htmlContainer?: HTMLElement): Promise<ConfigResult> => {
  const workspaceFileUri = vscode.Uri.file('/workspace.code-workspace');

  const vscodeApiConfig: MonacoVscodeApiConfig = {
    $type: 'extended',
    logLevel: LogLevel.Debug,
    serviceOverrides: {
      ...getKeybindingsServiceOverride(),
      ...getLifecycleServiceOverride(),
      ...getLocalizationServiceOverride(createDefaultLocaleConfiguration()),
      ...getNotificationsServiceOverride(),
      ...getBannerServiceOverride(),
      ...getStatusBarServiceOverride(),
      ...getTitleBarServiceOverride(),
      ...getExplorerServiceOverride(),
      ...getEnvironmentServiceOverride(),
      ...getSecretStorageServiceOverride(),
      ...getStorageServiceOverride(),
      ...getSearchServiceOverride(),
      ...getOutlineServiceOverride()
    },
    viewsConfig: {
      $type: 'ViewsService',
      htmlContainer,
      htmlAugmentationInstructions: defaultHtmlAugmentationInstructions,
      viewsInitFunc: defaultViewsInit
    },
    workspaceConfig: {
      enableWorkspaceTrust: true,
      windowIndicator: {
        label: 'rsm-demo',
        tooltip: '',
        command: ''
      },
      workspaceProvider: {
        trusted: true,
        async open() {
          window.open(window.location.href);
          return true;
        },
        workspace: {
          workspaceUri: workspaceFileUri
        }
      },
      configurationDefaults: {
        'window.title': 'rsm-demo${separator}${dirty}${activeEditorShort}'
      },
      productConfiguration: {
        nameShort: 'rsm-demo',
        nameLong: 'rsm-demo'
      }
    },
    userConfiguration: {
      json: JSON.stringify({
        'workbench.colorTheme': 'Default Dark Modern',
        'editor.wordBasedSuggestions': 'off',
        'editor.guides.bracketPairsHorizontal': true,
        'editor.experimental.asyncTokenization': true
      })
    },
    extensions: [
      {
        config: {
          name: 'rsm-demo',
          publisher: 'rsm',
          version: '1.0.0',
          engines: {
            vscode: '*'
          }
        }
      },
      { config: mumpsExtension }
    ],
    monacoWorkerFactory: configureDefaultWorkerFactory
  };

  const workspaceUri = vscode.Uri.file('/workspace');
  const mumpsUris = ['mumps1.m', 'mumps2.m', 'mumps3.m'].map((name) => vscode.Uri.file(`/workspace/${name}`));
  const fileSystemProvider = new InMemoryFileSystemProvider();
  const textEncoder = new TextEncoder();

  const options: IFileWriteOptions = {
    atomic: false,
    unlock: false,
    create: true,
    overwrite: true
  };
  await fileSystemProvider.mkdir(workspaceUri);
  for (const uri of mumpsUris) {
    await fileSystemProvider.writeFile(uri, textEncoder.encode(mumpsExampleContent), options);
  }
  await fileSystemProvider.writeFile(workspaceFileUri, textEncoder.encode(createDefaultWorkspaceContent('/workspace')), options);
  registerFileSystemOverlay(1, fileSystemProvider);

  const languageClientConfig: LanguageClientConfig = {
    languageId: mumps_language_id,
    connection: start_language_server(),
    clientOptions: {
      documentSelector: [mumps_language_id],
      workspaceFolder: {
        index: 0,
        name: 'workspace',
        uri: workspaceUri
      }
    }
  };

  return {
    vscodeApiConfig,
    workspaceFileUri,
    mumpsUris,
    languageClientConfig
  };
};
