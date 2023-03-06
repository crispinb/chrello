import { useState, useEffect } from 'preact/hooks';
import { invoke } from '@tauri-apps/api';
import ColumnComponent, { Column, Card } from './components/column';

export function App() {
  const [data, setData] = useState({});
  useEffect(() => {
    console.log('------> useEffect');
    const getData = async () => {
      const rustData = await invoke("get_dummy_data");
      console.log(`setting data to ${JSON.stringify(rustData)}`);
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
