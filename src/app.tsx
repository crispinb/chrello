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

export function App<FC>() {
  return (
    <div class="container">
      <h1 class="text-3xl font-bold">Chrello</h1>
      <p class="text-sky-700">Preact + tailwind</p>
    </div>
  );
}
