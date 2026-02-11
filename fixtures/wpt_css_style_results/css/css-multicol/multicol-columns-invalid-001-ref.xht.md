# css/css-multicol/multicol-columns-invalid-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-columns-invalid-001-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  table
  {
  background-color: yellow;
  border-spacing: 0px;
  border: gray solid 1em;
  font: 1.25em/1 serif;
  }

  td {padding: 0 1em 0 0;}

  img, td {vertical-align: top;}
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
