import { transformSync } from '@babel/core'
import type { PluginItem as BabelPluginItem } from '@babel/core'
import * as BabelTypes from '@babel/types'
import { createFilter } from 'vite'
import type { Plugin as VitePlugin, Rollup, FilterPattern } from 'vite'

import {
  TUONO_MAIN_PACKAGE,
  TUONO_DYNAMIC_FN_ID,
  TUONO_LAZY_FN_ID,
} from './constants'
import { isTuonoDynamicFnImported } from './utils'

/**
 * [SERVER build]
 * This plugin just removes the `dynamic` imported function from any tuono import
 */
const RemoveTuonoLazyImport: BabelPluginItem = {
  name: 'remove-tuono-lazy-import-plugin',
  visitor: {
    ImportDeclaration: (path) => {
      const importNode = path.node
      if (importNode.source.value !== TUONO_MAIN_PACKAGE) return

      path.traverse({
        ImportSpecifier: (importSpecifierPath) => {
          if (isTuonoDynamicFnImported(importSpecifierPath)) {
            importSpecifierPath.remove()
          }
        },
      })

      // If there are no specifiers left after traverse
      // remove the import to avoid unwanted side effects
      if (importNode.specifiers.length === 0) {
        path.remove()
      }
    },
  },
}

/**
 * [CLIENT build]
 * This plugin replace the `dynamic` function with the `__tuono__internal__lazyLoadComponent` one
 */
const ReplaceTuonoLazyImport: BabelPluginItem = {
  name: 'replace-tuono-lazy-import-plugin',
  visitor: {
    ImportSpecifier: (path) => {
      if (
        BabelTypes.isIdentifier(path.node.imported) &&
        isTuonoDynamicFnImported(path)
      ) {
        path.node.imported.name = TUONO_LAZY_FN_ID
      }
    },
  },
}

const turnLazyIntoStatic = {
  VariableDeclaration: (
    path: babel.NodePath<BabelTypes.VariableDeclaration>,
  ): void => {
    path.node.declarations.forEach((variableDeclarationNode) => {
      const init = variableDeclarationNode.init

      if (
        BabelTypes.isCallExpression(init) &&
        // ensures that the method call is `TUONO_DYNAMIC_FN_ID`
        BabelTypes.isIdentifier(init.callee, { name: TUONO_DYNAMIC_FN_ID }) &&
        // import name must be an identifier
        BabelTypes.isIdentifier(variableDeclarationNode.id) &&
        // check that the first function parameter is an arrow function
        BabelTypes.isArrowFunctionExpression(init.arguments[0])
      ) {
        const cmpImportFn = init.arguments[0]

        // ensures that the first parameter is a call expression (may be a block statement)
        if (!BabelTypes.isCallExpression(cmpImportFn.body)) return
        // ensures that the first parameter is a string literal (the import path)
        if (!BabelTypes.isStringLiteral(cmpImportFn.body.arguments[0])) return

        const importName = variableDeclarationNode.id.name
        const importPath = cmpImportFn.body.arguments[0].value

        if (importName && importPath) {
          const importDeclaration = BabelTypes.importDeclaration(
            [
              BabelTypes.importDefaultSpecifier(
                BabelTypes.identifier(importName),
              ),
            ],
            BabelTypes.stringLiteral(importPath),
          )

          path.replaceWith(importDeclaration)
        }
      }
    })
  },
}

/**
 * [SERVER build]
 * This plugin statically imports the lazy loaded components
 */
const TurnLazyIntoStaticImport: BabelPluginItem = {
  name: 'turn-lazy-into-static-import-plugin',
  visitor: {
    Program: (path) => {
      path.traverse({
        ImportSpecifier: (subPath) => {
          if (isTuonoDynamicFnImported(subPath)) {
            path.traverse(turnLazyIntoStatic)
          }
        },
      })
    },
  },
}

interface LazyLoadingPluginOptions {
  include: FilterPattern
}

export function LazyLoadingPlugin(
  options: LazyLoadingPluginOptions,
): VitePlugin {
  const { include } = options

  const filter = createFilter(include)

  return {
    name: 'vite-plugin-tuono-lazy-loading',
    enforce: 'pre',
    transform(code, id, opts): Rollup.TransformResult {
      if (!filter(id)) return

      /**
       * @todo we should exclude non tsx files from this transformation
       *       this might benefit build time avoiding running `includes` on non-tsx files.
       *       This can be executed using `_id` parameter
       *       which is the filepath that is being processed
       */

      if (
        code.includes(TUONO_DYNAMIC_FN_ID) &&
        code.includes(TUONO_MAIN_PACKAGE)
      ) {
        const plugins: Array<BabelPluginItem> = [
          ['@babel/plugin-syntax-jsx', {}],
          ['@babel/plugin-syntax-typescript', { isTSX: true }],
        ]

        if (opts?.ssr) {
          plugins.push(RemoveTuonoLazyImport, TurnLazyIntoStaticImport)
        } else {
          plugins.push(ReplaceTuonoLazyImport)
        }

        const res = transformSync(code, { plugins })

        return res?.code
      }

      return code
    },
  }
}
