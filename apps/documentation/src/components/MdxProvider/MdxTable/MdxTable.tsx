import type { DetailedHTMLProps, TableHTMLAttributes } from 'react'
import { Table } from '@mantine/core'

type MdxTableProps = DetailedHTMLProps<
  TableHTMLAttributes<HTMLTableElement>,
  HTMLTableElement
>

function MdxTable(props: MdxTableProps) {
  return (
    <Table highlightOnHover stickyHeader stickyHeaderOffset={60} {...props}>
      {props.children}
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
