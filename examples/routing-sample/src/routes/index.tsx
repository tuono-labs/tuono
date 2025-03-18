import type { JSX } from 'react'
import type { TuonoProps } from 'tuono'

interface IndexProps {
  data: Array<String>
}

export default function IndexPage({
  data,
}: TuonoProps<IndexProps>): JSX.Element | null {
  console.log(data);

  if (!data) return null

  return (
    <>
      <header className="header">
        <h1>Chuck Norris Facts</h1>
      </header>
      <ul style={{ flexWrap: "wrap", display: "flex", gap: 10, listStyle: "none", padding: 0 }}>
        {data.map((category, index) => (
          <li key={index} style={{ marginLeft: "16px" }}>
            <a href={`/category/${category}`}>
              {category}
            </a>
          </li>
        ))}
      </ul>
    </>
  );
}