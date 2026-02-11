# css/css-multicol/multicol-nested-005.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-nested-005.xht"
}
```

## style[0]

```css
<![CDATA[
  body > div
  {
  column-gap: 1em;
  font: 1.25em/1 Ahem;
  width: 41em;
  }

  /*

  N == 3;

  W == max(0, (available-width - ((N - 1) * column-gap)) / N);
  W == max(0, (41em - ((3 - 1) * 1em)) / 3);
  W == max(0, (41em - (2 * 1em)) / 3);
  W == max(0, (41em - 2em) / 3);
  W == max(0, 39em / 3);
  W == max(0, 13em);
  W == 13em;

  The height of column rule depends on number of line boxes in
  each outer column box which depends on number of line boxes
  in each inner column box. So:

    13em : width of each outer column box
   -
     2em : horizontal margin of each div inside
   =======
    11em : width of each inner multi-column elements

  N == 3;

  W == max(0, (available-width - ((N - 1) * column-gap)) / N);
  W == max(0, (11em - ((3 - 1) * 1em)) / 3);
  W == max(0, (11em - (2 * 1em)) / 3);
  W == max(0, (11em - 2em) / 3);
  W == max(0, 9em / 3);
  W == max(0, 3em);
  W == 3em;

  So, each duo of 'a', 'm' and 'x' should fill one and only 1 line
  box. There are 8 duos; therefore, the first 2 inner
  column boxes should use 3 line boxes and the 3rd inner
  column box should be using 2 line boxes.

  So, the height of the 2 blue column rules should be 60px.

  */

  div
  {
  background-color: yellow;
  color: black;
  font-size: 1em;
  margin: 1em;
  orphans: 1;
  widows: 1;

  column-count: 3;
  }

  div > div:first-child {margin-top: 0;}
  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
