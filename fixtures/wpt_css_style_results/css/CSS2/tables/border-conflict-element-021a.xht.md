# css/CSS2/tables/border-conflict-element-021a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-conflict-element-021a.xht"
}
```

## style[0]

```css
<![CDATA[
  table#test {border: red solid 1em;}

  table
  {
  border-collapse: collapse;
  color: white;
  font: 1.25em/1 Ahem;
  margin: auto auto 2em 2em;
  }

  thead, tbody, tfoot {border: solid 1em;}

  thead {border-color: blue;}
  tbody {border-color: yellow;}
  tfoot {border-color: orange;}

  td {padding: 0px;}

  table#reference > tbody {border: red none 0em;}

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
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
