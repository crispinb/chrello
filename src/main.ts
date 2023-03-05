import { invoke } from "@tauri-apps/api/tauri";
import Alpine from 'alpinejs';

const dummyData = {
  columns: [
    {
      name: "colname1",
      id: 1,
      cards: [
        {
          id: 1,
          content: "this is the card1",
        },
        {
          id: 2,
          content: "this is more card2",
        }
      ]},
      {
        name: "colname2",
        id: 2,
        cards: [
          {
            id: 1,
            content: "this is the card-2/3",
          },
          {
            id: 2,
            content: "this is more ca2/4",
          }
        ]},
{
        name: "colname3",
        id: 2,
        cards: [
          {
            id: 1,
            content: "this is the card-2/3",
          },
          {
            id: 2,
            content: "this is more ca2/4",
          }
        ]}
  ]
};

async function getData(): Promise<any> {
  console.log('--> called');
  return dummyData;
}

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
  return await (invoke("get_lists"));
}

Alpine.data('api', () => {
  return {
    init() { console.log('cvapi init') },
   get data () { return getData() },
    lists: getLists,
    tasks: getTasks,
    cardTap: cardTap,
  }
});

Alpine.start();

