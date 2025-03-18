import type { JSX } from 'react'
import type { TuonoProps } from 'tuono'

interface CategoryProps {
    data: {
        categories: Array<string>,
        created_at: string,
        icon_url: string,
        id: string,
        updated_at: string,
        url: string,
        value: string
    }
}

export default function CategoryPage({
    data,
}: TuonoProps<CategoryProps>): JSX.Element | null {
    console.log(data);

    if (!data) return null

    return (
        <>
            <header className="header">
                <h1>Chuck Norris Facts</h1>
            </header>
            <div>
                <img src={data.icon_url} className="rust" />
                {
                    <h2>{data.value}</h2>
                }
            </div>
        </>
    )
}