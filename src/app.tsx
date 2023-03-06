import { useState, useEffect } from 'preact/hooks';
import ColumnComponent, { Column, Card } from './components/column';
import { UICard, UIColumn, UIBoard, getDummyData } from './bindings';

export function App() {
  // there must be a better way to coerce the useState type?
  const emptyBoard: UIBoard = { columns: [] };
  const [data, setData] = useState(emptyBoard);
  useEffect(() => {
    console.log('------> useEffect');
    const getData = async () => {
      const rustData: UIBoard = await getDummyData();
      console.log(`retturnd data is: ${JSON.stringify(rustData)}`);
      setData(rustData);
    }

    getData();
  }, []);

  return (
    <>
      <h1 class='text-xl font-semibold text-blue-500'>Chrello</h1>
      <div class='flex m-4'>
        {data.columns ?
          data.columns.map(col => <ColumnComponent {...col} />) :
          <p>(no data)</p>
        }
      </div>
    </>
  );
}
