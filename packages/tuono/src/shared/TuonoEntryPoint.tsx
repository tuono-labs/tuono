import { type JSX, type ReactNode, StrictMode } from 'react'

interface TuonoEntryPointProps {
  children: ReactNode
}

export function TuonoEntryPoint({
  children,
}: TuonoEntryPointProps): JSX.Element {
  return <StrictMode>{children}</StrictMode>
}
