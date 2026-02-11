# css/css-pseudo/marker-content-002.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-content-002.html"
}
```

## style[0]

```css

li::marker {
  content: "a" counters(list-item, ".") "b";
}
span { font-size: 32pt; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
