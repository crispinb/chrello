import { UICard } from '../bindings';

export default function CardComponent({ id, content }: UICard) {
  return (
    <div id={`card-${id}`} class='bg-slate-50 p-2 rounded border-b border-grey' >
      {content}
    </div>
  )
}

