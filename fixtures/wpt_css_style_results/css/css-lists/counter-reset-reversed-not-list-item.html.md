# css/css-lists/counter-reset-reversed-not-list-item.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/counter-reset-reversed-not-list-item.html"
}
```

## style[0]

```css

  span { counter-increment: not-list-item -2; }
  .reset-reversed { counter-reset: reversed(not-list-item) }
  .result::before { content: counter(not-list-item) }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “counter-reset”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
