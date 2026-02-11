# css/css-contain/contain-layout-breaks-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-layout-breaks-001.html"
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
  line-height: 1;
  height: 4em;
  column-fill: auto;
}
div > div:last-of-type {
  break-before: column;
}
#test > div {
  contain: layout;
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
