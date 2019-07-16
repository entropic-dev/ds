import fs from 'fs';
import Module from 'module';

export function overrideNode() {
  let babel: any;
  let reactPlugin: any;

  // @ts-ignore apparently _extensions doesn't exist on module? Not sure what this is doing...
  Module._extensions['.jsx'] = (module: any, filename: string) => {
    const content = fs.readFileSync(filename, 'utf8');
    if (!babel) {
      babel = require('babel-core');
    }
    if (!reactPlugin) {
      reactPlugin = require('babel-plugin-transform-react-jsx');
    }
    const { code } = babel.transform(content, {
      plugins: [
        [
          reactPlugin,
          {
            pragma: 'h',
            useBuiltIns: true
          }
        ]
      ]
    });
    module._compile(code, filename);
  };

  let tsPlugin: any;

  // @ts-ignore
  Module._extensions['.ts'] = (module: any, filename: string) => {
    const content = fs.readFileSync(filename, 'utf8');
    if (!babel) {
      babel = require('babel-core');
    }
    if (!tsPlugin) {
      tsPlugin = require('babel-plugin-transform-typescript');
    }
    const { code } = babel.transform(content, {
      plugins: [tsPlugin]
    });
    module._compile(code, filename);
  };

  // @ts-ignore
  Module._extensions['.tsx'] = Module._extensions['.ts'];
}
