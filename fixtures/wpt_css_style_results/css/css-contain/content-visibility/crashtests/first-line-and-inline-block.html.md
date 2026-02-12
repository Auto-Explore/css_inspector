# css/css-contain/content-visibility/crashtests/first-line-and-inline-block.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/crashtests/first-line-and-inline-block.html"
}
```

## style[0]

```css

div, span {
  content-visibility: auto;
  contain-intrinsic-width: auto 100vw;
}
div::first-line {
  color: blue;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
