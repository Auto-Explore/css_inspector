# css/css-multicol/multicol-fill-000.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-fill-000.xht"
}
```

## style[0]

```css
<![CDATA[
  div.test, table#reference
  {
  background-color: yellow;
  color: lime;
  margin: 1em 0em;
  font: 1.25em/1 Ahem;
  width: 18em;
  }

  div.test
  {
  columns: 3;
  column-fill: balance;
  column-gap: 0em;

  /*

  N == 3;

  W == 6em;

  */

  orphans: 1;
  widows: 1;
  }

  div.test > p {margin: 0em;}

  table
  {
  border-spacing: 0em;
  table-layout: fixed;
  }

  td {padding: 0em;}
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
