# css/css-multicol/multicol-columns-invalid-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-columns-invalid-001.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  background-color: yellow;
  border: gray solid 1em;
  color: black;
  font: 1.25em/1 Ahem;
  orphans: 1;
  widows: 1;
  width: 12em;

  column-count: 4;
  column-gap: 0;
  columns: 8 normal;
  }

  span {color: blue;}
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
