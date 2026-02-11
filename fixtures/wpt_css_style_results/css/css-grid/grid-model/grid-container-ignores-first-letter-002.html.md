# css/css-grid/grid-model/grid-container-ignores-first-letter-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-model/grid-container-ignores-first-letter-002.html"
}
```

## style[0]

```css

  /* The combination of button, grid, nested inlines, and ::first-letter crashes Blink */
  button { display: grid }
  button::first-letter { color: red }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
