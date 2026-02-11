# css/CSS2/tables/border-conflict-element-001e-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-conflict-element-001e-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  table
  {
  border-collapse: collapse;
  color: white;
  font: 1.25em/1 Ahem;
  margin: auto auto 2em 2em;
  }

  td {padding: 0px;}

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
