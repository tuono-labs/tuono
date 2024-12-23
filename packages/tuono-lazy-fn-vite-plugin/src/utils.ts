import {
  isIdentifier,
  isImportDeclaration,
  isImportSpecifier,
} from '@babel/types'

import { TUONO_MAIN_PACKAGE, TUONO_DYNAMIC_FN_ID } from './constants'

/**
 * By a given AST Node path returns true if the path involves an import specifier
 * importing {@link TUONO_DYNAMIC_FN_ID}
 */
export const isTuonoDynamicFnImported = (path: babel.NodePath): boolean => {
  // If the node isn't an import declaration there is no need to process it
  if (!isImportDeclaration(path.parentPath?.node)) return false

  // if the import doesn't import from 'tuono' we don't need to process it
  if (path.parentPath.node.source.value !== TUONO_MAIN_PACKAGE) return false

  // ensure that we are processing an import specifier
  if (!isImportSpecifier(path.node)) return false

  // finally check if the imported item is `TUONO_DYNAMIC_FN_ID`
  return isIdentifier(path.node.imported, { name: TUONO_DYNAMIC_FN_ID })
}
