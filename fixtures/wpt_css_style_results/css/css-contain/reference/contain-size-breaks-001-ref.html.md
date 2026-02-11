# css/css-contain/reference/contain-size-breaks-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/reference/contain-size-breaks-001-ref.html"
}
```

## style[0]

```css

article {
  height: 2.5em;
  column-gap: 0;
  columns: 3 1em;
  width: 3em;
  column-fill: auto;

  font-size: 40px;
  font-family: ahem;
  line-height: 1;
}
div {
  background: orange;
  padding-top: 5em;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “columns”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
