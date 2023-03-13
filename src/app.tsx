import * as commands from './bindings.ts';
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
  const [count, setCount] = useState<number>(0);

  useEffect(() => {
    const start_timer = async () => {
      await commands.initTimer();
    };
      start_timer()
  }, []);

  useEffect(() => {
    console.log('--> state useEffect');
    const getState = async () => {
      await appWindow.listen('new-state', (event) => {
        setCount(event.payload);
      });
    };
    getState();
  }, []);

  useEffect(() => {
    const listen = async () => {
      const unlisten = await appWindow.listen('test-event', (event) => {
        console.log('got event');
        console.log(event.payload);
        setBoard(event.payload as UIBoard);
      });
      console.log(`unlisten handler: ${JSON.stringify(unlisten)}`);
      return unlisten;
    }
    console.log("trying to listen ..");
    const clear_listener = listen();
    console.log("returned henalder", clear_listener);
  }, [board]);


  return (
    <>
      <h1 class='text-xl font-semibold text-blue-500'>Chrello</h1>
      <div class='flex m-4'>
        {board.columns.map(col => <ColumnComponent {...col} />)}
      </div>
      <span class='font-bold text-green-800'> Count: {count}</span>
      <button onClick={() => commands.setState(count + 1)}>Inc count</button>
    </>
  );
}
