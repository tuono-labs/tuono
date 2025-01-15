import type { ReactNode } from 'react'
import { Head } from 'tuono'

interface MetaTagsProps {
  title: string
  description?: string
  type?: 'website' | 'article' | 'book' | 'video.movie' | 'music.song'
  canonical: string
}

export default function MetaTags({
  title,
  description,
  type = 'article',
  canonical,
}: MetaTagsProps): ReactNode {
  return (
    <Head>
      <title>{title}</title>
      <link rel="canonical" href={canonical} />
      <meta
        name="description"
        content={
          description ??
          'The technologies we love seamlessly working together to unleash the highest web performance ever met on React'
        }
      />
      <meta
        name="keywords"
        content="ReactJs, Typescript, Rust, JavaScript, Fullstack framework, Web development"
      />
      <meta property="og:title" content={title} />
      <meta property="og:type" content={type} />
      <meta property="og:image" content="https://tuono.dev/og-cover.png" />
      <meta property="og:url" content={canonical} />
      <meta property="og:description" content={description} />
      <meta property="og:site_name" content="Tuono" />
      <meta property="og:locale" content="en_US" />
      <meta name="twitter:card" content="summary_large_image" />
      <meta name="twitter:site" content="@tuono_dev" />
      <meta name="twitter:title" content={title} />
      <meta name="twitter:description" content={description} />
      <meta name="twitter:image" content="https://tuono.dev/og-cover.png" />
    </Head>
  )
}
