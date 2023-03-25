import * as commands from './bindings';
import { appWindow } from '@tauri-apps/api/window'
import { useState, useEffect } from 'preact/hooks';
import ColumnComponent, { Column, Card } from './components/column';
import { UICard, UIColumn, UIBoard, getDummyData } from './bindings';

// rerendering from the top with UIBoard
// Prop equality is a single-level shallow exact compare
// We need to think through for each component
// whether the return value from Rust will cause a rerender
// https://dev.to/sheepysean/equality-in-reactjs-the-shallowequal-2bci
// If it does when it shouldn't we might be able to rearrnage the components
// and/or state
// oterhwise look at useMemo and Usecallback
//
export function App() {
  // note the setters are only called from our Rust callbacks in useEffects
  const [board, setBoard] = useState<UIBoard>({ columns: [] });

  // fire off initial data load
  useEffect(() => {
    console.log('---> load data');
    const loadData = async () => {
      await commands.loadInitialData();
    }

    loadData();
  }, []);

  useEffect(() => {
    const listenForBoardData = async () => {
      const unlisten = await appWindow.listen('initial-data', (event) => {
        console.log('got dat');
        console.log(event.payload);
        setBoard(event.payload as UIBoard);
      });
      console.log(`unlisten handler: ${JSON.stringify(unlisten)}`);
      return unlisten;
    }
    const clear_listener = listenForBoardData();
    console.log("clearer: ", clear_listener);
  }, [board]);


  return (
    <>
      <h1 class='text-xl font-semibold text-blue-500'>Chrello</h1>
      <div class='flex m-4'>
        {board.columns.map(col => <ColumnComponent {...col} />)}
      </div>
    </>
  );
}
