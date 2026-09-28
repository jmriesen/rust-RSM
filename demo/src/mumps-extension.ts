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

export const mumpsExampleContent = `tag w "before loop",!
 f i=1:1:5 w "foo "
 w !,"after loop"
 w "foo" 
 q  
`;
