# css/css-multicol/multicol-span-000.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-000.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  background-color: yellow;
  color: lime;
  font: 1.25em/1 Ahem;
  orphans: 1;
  widows: 1;
  width: 10em;
  }

  div#test, div.test2
  {
  columns: 2;
  column-fill: balance;
  column-gap: 0em;

  /*

  N == 2;

  W == 5em;

  */
  }

  div#test, div#reference {margin-bottom: 0.5em;}

  div#column-span
  {
  column-span: all;
  font: inherit;
  }

  img
  {
  padding-right: 4em;
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
