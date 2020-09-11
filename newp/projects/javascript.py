# flake8: noqa

template = {
    "__desc": "a Javascript project using Babel and TypeScript",
    "README.md": """
# {{name}}

{{description}}

## Development Commands

`npm ts` - Runs TypeScript checks
`npm tests` - Runs tests
`npm checks` - Runs both of the above
`npm build` - Transpiles code from `src` into standard Javascript in `lib`



""",
    "package.json": """
  {
  "name": "{{camel_case_name}}",
  "version": "0.1.0",
  "description": "{{ description }}",
  "bin": {
    "{{ name }}": "./bin/{{ camel_case_name }}.js"
  },
  "scripts": {
    "checks": "npm run lint && npm run tests",
    "lint": "tsc --project tsconfig.json --outDir output/tsOutput --noEmit",
    "build": "babel src --out-dir lib --extensions '.ts,.tsx'",
    "tests": "jest"
  },
  "author": "{{ author }}",
  "license": "{{ license }}",
  "jest": {
    "verbose": true,
    "modulePaths": [
      "src"
    ]
  },
  "devDependencies": {
    "@babel/cli": "^7.10.5",
    "@babel/core": "^7.10.5",
    "@babel/plugin-proposal-class-properties": "^7.10.4",
    "@babel/plugin-proposal-object-rest-spread": "^7.10.4",
    "babel-plugin-module-resolver": "^4.0.0",
    "@babel/preset-env": "^7.10.4",
    "@babel/preset-typescript": "^7.10.4",
    "babel-jest": "^26.1.0",
    "jest": "^26.1.0",
    "ts-jest": "^26.3.0",
    "@types/jest": "^26.0.13",
    "typescript": "^4.0.2"
  }
}
""",
    "tsconfig.json": """{
  "compilerOptions": {
    "skipLibCheck": true,
    "target": "es6",
    "noEmit": true,
    "noImplicitAny": true,
    "noImplicitThis": true,
    "strictNullChecks": false,
    "types": [
        "node", "jest"
    ],
  },
  "exclude": [
    "node_modules"
  ],
  "baseUrl": "/",
  "paths": {
    "*": [ "src/*", "tests/*" ]
  },
  "include": [
    "src/**/*.ts",
    "tests/**/*.ts"
  ]
}
""",
    "babel.config.json": """{
  "presets": [
    "@babel/preset-env",
    "@babel/preset-typescript"
  ],
  "plugins": [
    "@babel/proposal-class-properties",
    "@babel/proposal-object-rest-spread",
    [
      "module-resolver",
      {
        "root": [
          "./src"
        ]
      }
    ]
  ]
}
""",
    "src/{{camel_case_name}}.ts": """
export const hello = () => {
    return 42;
};
""",
    "src/__tests__/test{{pascal_case_name}}.ts": """
import { hello } from "{{ camel_case_name }}";

test("example test", () => {
    expect(hello()).toBe(42);
});
""",
    "bin/{{ camel_case_name }}.js": """#!/usr/bin/env node

console.log("hi!");
""",
}
