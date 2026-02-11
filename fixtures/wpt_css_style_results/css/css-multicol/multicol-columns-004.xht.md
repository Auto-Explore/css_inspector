# css/css-multicol/multicol-columns-004.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-columns-004.xht"
}
```

## style[0]

```css
<![CDATA[
  body {width: 600px;}

  div
  {
  background-color: yellow;
  color: black;
  font: 1.25em/1 Ahem;
  orphans: 1;
  widows: 1;

  columns: auto 100px;
  column-gap: 0;
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
      "message": "Invalid value for property “columns”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
