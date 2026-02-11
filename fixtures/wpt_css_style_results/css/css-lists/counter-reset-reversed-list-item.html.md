# css/css-lists/counter-reset-reversed-list-item.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/counter-reset-reversed-list-item.html"
}
```

## style[0]

```css

  .li { display: list-item; }
  .reset-reversed { counter-reset: reversed(list-item) }
  .result::before { content: counter(list-item) }
  ::marker { content: none; }
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
