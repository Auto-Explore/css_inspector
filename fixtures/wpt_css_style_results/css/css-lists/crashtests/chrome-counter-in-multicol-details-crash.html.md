# css/css-lists/crashtests/chrome-counter-in-multicol-details-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/crashtests/chrome-counter-in-multicol-details-crash.html"
}
```

## style[0]

```css

  #counter { counter-reset:counter; }
  #counter::after { content: counter(foo); }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
