# css/css-contain/reference/contain-style-breaks-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/reference/contain-style-breaks-001-ref.html"
}
```

## style[0]

```css

article {
  columns: 2 1ch;
  column-gap: 0;
  float: left;
  font-family: monospace;
  margin-right: 3em;
}
div {
  page-break-inside: avoid;
  break-inside: avoid;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “columns”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
