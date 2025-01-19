import type { DetailedHTMLProps, TableHTMLAttributes, JSX } from 'react'
import { Table, ScrollArea } from '@mantine/core'

type MdxTableProps = DetailedHTMLProps<
  TableHTMLAttributes<HTMLTableElement>,
  HTMLTableElement
>

function MdxTable(props: MdxTableProps): JSX.Element {
  const { children, ...rest } = props
  return (
    <ScrollArea type="auto">
      <Table highlightOnHover {...rest}>
        {children}
      </Table>
    </ScrollArea>
  )
}

MdxTable.Thead = Table.Thead
MdxTable.Tbody = Table.Tbody
MdxTable.Tr = Table.Tr
MdxTable.Th = Table.Th
MdxTable.Td = Table.Td
MdxTable.Caption = Table.Caption

export default MdxTable
