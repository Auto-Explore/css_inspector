# css/css-writing-modes/link-writing-mode-dependency-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/link-writing-mode-dependency-crash.html"
}
```

## style[0]

```css

a {
  border-inline-start: currentColor;
}
:link {
  border-inline-start-color: #cba7;
  writing-mode: sideways-lr;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
