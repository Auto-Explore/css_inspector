# css/css-pseudo/marker-content-024.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-content-024.html"
}
```

## style[0]

```css

ol {
  margin: 0;
  padding-left: 25px;
  font: 25px/1 Ahem;
}
li {
  width: 150px;
  column-count: 2;
  column-gap: 0;
  text-emphasis-style: circle;
  -webkit-text-emphasis-style: circle;
}
li + li {
  list-style-position: inside;
}
::marker {
  content: 'X';
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “-webkit-text-emphasis-style”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
