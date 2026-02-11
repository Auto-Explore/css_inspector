# css/css-highlight-api/crashtests/highlight-first-letter-float-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/crashtests/highlight-first-letter-float-crash.html"
}
```

## style[0]

```css

div::first-letter {
  float: right;
}
::highlight(test) { background-color: purple; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
