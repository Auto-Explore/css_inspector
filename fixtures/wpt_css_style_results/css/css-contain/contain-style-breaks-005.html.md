# css/css-contain/contain-style-breaks-005.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-style-breaks-005.html"
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
div > div:first-of-type {
  break-after: column;
}
#test > div {
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
