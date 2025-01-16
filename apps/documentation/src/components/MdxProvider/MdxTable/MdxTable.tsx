import type { DetailedHTMLProps, TableHTMLAttributes } from 'react'
import { Table } from '@mantine/core'
import type React from 'react'

type MdxTableProps = DetailedHTMLProps<
  TableHTMLAttributes<HTMLTableElement>,
  HTMLTableElement
>

function MdxTable(props: MdxTableProps): React.JSX.Element {
  const { children, ...rest } = props
  return (
    <Table highlightOnHover stickyHeader stickyHeaderOffset={60} {...rest}>
      {children}
    </Table>
  )
}

MdxTable.Thead = Table.Thead
MdxTable.Tbody = Table.Tbody
MdxTable.Tr = Table.Tr
MdxTable.Th = Table.Th
MdxTable.Td = Table.Td
MdxTable.Caption = Table.Caption

export default MdxTable
