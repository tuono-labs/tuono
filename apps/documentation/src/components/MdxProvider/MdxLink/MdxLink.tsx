import type { JSX, AnchorHTMLAttributes } from 'react'
import { Button } from '@mantine/core'
import { Link } from 'tuono'
import { IconExternalLink } from '@tabler/icons-react'

export default function MdxLink(
  props: AnchorHTMLAttributes<HTMLAnchorElement>,
): JSX.Element {
  if (props.href?.startsWith('http') || props.href?.startsWith('mailto')) {
    return (
      <Button
        component="a"
        {...props}
        target="_blank"
        rightSection={
          <IconExternalLink size="16px" style={{ marginLeft: -5 }} />
        }
        variant="transparent"
        style={{ height: '20px' }}
        mt={-2}
        p={0}
      />
    )
  }
  return (
    <Button
      component={Link}
      {...props}
      target="_blank"
      variant="transparent"
      style={{ height: '20px' }}
      mt={-2}
      p={0}
    />
  )
}
