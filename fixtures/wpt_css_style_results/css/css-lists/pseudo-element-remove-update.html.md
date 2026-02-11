# css/css-lists/pseudo-element-remove-update.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/pseudo-element-remove-update.html"
}
```

## style[0]

```css

body {
  counter-reset: myCounter;
}
div::before {
  content: counter(myCounter);
  counter-increment: myCounter;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
