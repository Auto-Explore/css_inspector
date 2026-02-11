# css/css-lists/counter-reset-reversed-list-item-start.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/counter-reset-reversed-list-item-start.html"
}
```

## style[0]

```css

  .li { display: list-item; }
  .reset-reversed { counter-reset: reversed(list-item) 10 }
  .result::before { content: counter(list-item) }
  ::marker { content: none; }
  .li:not(.result) { height: 0 }
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
