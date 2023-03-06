import CardComponent, { Card} from './card';

export interface Column {
  id: number,
  name: string
  cards: Array<Card>
}

export default function ColumnComponent({ id, name, cards }: Column) {
  console.log(`cards: ${JSON.stringify(cards)}`);

  return (
    <div id={`col-${id}`} class='bg-slate-200 rounded px-4 pb-8 items-start font-bold'>
      {name} (column # {id})
      <div class="rounded bg-slate-100  flex-no-shrink w-64 p-2 mr-3 font-normal">
        {cards.map(card => (
          <CardComponent {...card} />
        ))}
      </div>
    </div>
  )
}
