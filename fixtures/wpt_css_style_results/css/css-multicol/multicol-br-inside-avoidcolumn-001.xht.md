# css/css-multicol/multicol-br-inside-avoidcolumn-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-br-inside-avoidcolumn-001.xht"
}
```

## style[0]

```css
<![CDATA[
.multicol {
  column-count: 2;
  column-gap: 0;
  column-fill: auto;
  overflow: hidden;
  width: 200px;
  height: 300px;
}
.multicol > div {
  height: 200px;
  break-inside: avoid-column;
  background: green;
}
.multicol > div.red {
  background:red;
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
