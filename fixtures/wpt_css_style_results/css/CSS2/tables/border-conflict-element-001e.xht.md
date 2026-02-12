# css/CSS2/tables/border-conflict-element-001e.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-conflict-element-001e.xht"
}
```

## style[0]

```css
<![CDATA[
  table#test
  {
  border: red solid 1em;
  }

  table
  {
  border-collapse: collapse;
  font: 1.25em/1 serif;
  margin: auto auto 2em 2em;
  }

  table#test td
  {
  border: solid 1em;
  padding: 0.5em;
  }

  td.blue {color: blue;}

  td.yellow {color: yellow;}

  td.orange {color: orange;}

  table#reference td {padding: 0em;}

  img
  {
  height: 1em;
  vertical-align: bottom;
  /*
  With 'vertical-align: bottom', swatch-[color] images "sit"
  at the bottom of the cell's line box and not on its baseline
  */
  width: 1em;
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
