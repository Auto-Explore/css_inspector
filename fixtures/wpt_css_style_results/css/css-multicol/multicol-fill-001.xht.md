# css/css-multicol/multicol-fill-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-fill-001.xht"
}
```

## style[0]

```css
<![CDATA[
  div#test, table#reference
  {
  background-color: yellow;
  color: lime;
  font: 1.25em/1 Ahem;
  height: 5em;
  margin: 1em 0em;
  width: 20em;
  }

  div#test
  {
  columns: 2;
  column-fill: balance;
  column-gap: 0em;

  /*

  N == 2;

  W == 10em;

  */
  }

  table
  {
  border-spacing: 0em;
  table-layout: fixed;
  }

  td
  {
  padding: 0em;
  vertical-align: top;
  }
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
