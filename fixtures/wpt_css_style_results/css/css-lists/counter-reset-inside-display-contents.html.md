# css/css-lists/counter-reset-inside-display-contents.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/counter-reset-inside-display-contents.html"
}
```

## style[0]

```css

  .inc { counter-increment: x }
  .reset-6 { counter-reset: x 6 }
  .reset-666 { counter-reset: x 6 }
  .contents { display: contents }
  .result::before { content: counter(x) }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
