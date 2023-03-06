export interface Card {
  id: number,
  content: string
}

export default function CardComponent({ id, content }: Card) {
  return (
    <div id={`card-${id}`} class='bg-slate-50 p-2 rounded border-b border-grey' >
      {content}
    </div>
  )
}

