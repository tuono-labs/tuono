import type { DetailedHTMLProps, TableHTMLAttributes } from 'react'
import { Table } from '@mantine/core'
import type React from 'react'

import styles from './MdxTable.module.css'

type MdxTableProps = DetailedHTMLProps<
  TableHTMLAttributes<HTMLTableElement>,
  HTMLTableElement
>

function MdxTable(props: MdxTableProps): React.JSX.Element {
  const { children, ...rest } = props
  return (
    <div className={styles.tableWrapper}>
      <Table className={styles.table} highlightOnHover {...rest}>
        {children}
      </Table>
    </div>
  )
}

MdxTable.Thead = Table.Thead
MdxTable.Tbody = Table.Tbody
MdxTable.Tr = ({
  children,
  ...rest
}: React.ComponentProps<typeof Table.Tr>) => (
  <Table.Tr className={styles.tableRow} {...rest}>
    {children}
  </Table.Tr>
)
MdxTable.Th = Table.Th
MdxTable.Td = Table.Td
MdxTable.Caption = Table.Caption

export default MdxTable
