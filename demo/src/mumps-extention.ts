export const mumps_language_id = 'mumps';

export const mumpsExtension = {
  name: 'mumps-language',
  publisher: 'rsm',
  version: '1.0.0',
  engines: { vscode: '*' },
  contributes: {
    languages: [
      {
        id: mumps_language_id,
        extensions: ['.m'],
        aliases: ['MUMPS']
      }
    ]
  }
};

export const mumpsExampleConntent = `tag s asdf=90
  write foo=90
  s foo="test"
  set foo("test")=90
`;
