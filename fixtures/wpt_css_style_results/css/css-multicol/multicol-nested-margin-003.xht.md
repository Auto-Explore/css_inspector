# css/css-multicol/multicol-nested-margin-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-nested-margin-003.xht"
}
```

## style[0]

```css
<![CDATA[
  body > div
  {
  font: 1.25em/1 Ahem;
  margin: 1em;
  width: 41em;
  /*
  This test requires a viewport width of 860px
  */
  }

  div
  {
  background-color: yellow;
  color: black;
  margin: 1em 1em 0;
  orphans: 1;
  widows: 1;

  column-count: 3;
  column-gap: 1em;
  }

  /*

  N == 3;

  43em - 2em (horizontal margins) == available-width

  W == max(0, (available-width - ((N - 1) * column-gap)) / N);
  W == max(0, (41em - ((3 - 1) * 1em)) / 3);
  W == max(0, (41em - (2 * 1em)) / 3);
  W == max(0, (41em - 2em) / 3);
  W == max(0, 39em / 3);
  W == max(0, 13em);
  W == 13em;

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

  */

  div div:nth-child(2) {color: pink;}

  div div:nth-child(3) {color: blue;}
  ]]>
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
