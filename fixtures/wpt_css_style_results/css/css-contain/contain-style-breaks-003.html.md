# css/css-contain/contain-style-breaks-003.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-style-breaks-003.html"
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
article > div {
  page-break-inside: avoid;
  break-inside: avoid;
}
#test div > div {
  contain: style;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
