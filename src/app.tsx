import ColumnComponent, { Column, Card } from './components/column';

const dummyData = {
  columns: [
    {
      name: "col1",
      id: 1,
      cards: [
        {
          id: 1,
          content: " card1",
        },
        {
          id: 2,
          content: " card2",
        }
      ]
    },
    {
      name: "colname2",
      id: 2,
      cards: [
        {
          id: 1,
          content: " card-2/3",
        },
        {
          id: 2,
          content: " ca2/4",
        }
      ]
    },
    {
      name: "colname3",
      id: 2,
      cards: [
        {
          id: 1,
          content: " card-3/1",
        },
        {
          id: 2,
          content: "card 3/2",
        }
      ]
    }
  ]
};

function getData() {
  return dummyData;
}

export function App() {
  const columns = getData().columns;
  return (
    <>
      <h1 class='text-xl font-semibold text-blue-500'>Chrello</h1>
      <div class='flex m-4'>
        {columns.map(col => (
        <ColumnComponent {...col } />
        ))}
      </div>
    </>
  );
}
