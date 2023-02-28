import { invoke } from "@tauri-apps/api/tauri";
import Alpine from 'alpinejs';

async function cardTap(item_id) : Promise<String> {
  console.log(`tapped: ${typeof item_id}`);
  console.log(`--> card tapped ${JSON.stringify(item_id)}`);
  return await invoke("item_chosen", {itemId: item_id});
}

async function getTasks(): Promise<Array<any>> {
  console.log("--> getTasks()");
  return await (invoke("get_tasks"));
}

async function getLists(): Promise<Array<any>> {
  console.log("--> getLists()");
  return await (invoke("get_lists"));
}

Alpine.data('cvapi', () => {
  return {
    init() { console.log('cvapi init') },
    lists: getLists,
    tasks: getTasks,
    cardTap: cardTap,
  }
});

Alpine.start();

