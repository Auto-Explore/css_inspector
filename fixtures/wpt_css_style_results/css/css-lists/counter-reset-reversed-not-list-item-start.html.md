# css/css-lists/counter-reset-reversed-not-list-item-start.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/counter-reset-reversed-not-list-item-start.html"
}
```

## style[0]

```css

  .reset-reversed { counter-reset: reversed(not-list-item) 7 }
  .result::before { content: counter(not-list-item) }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
