# css/css-overflow/column-updates-computed-style.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/column-updates-computed-style.html"
}
```

## style[0]

```css

.green-column::column { color: red; }
.red-column::column { color: green; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
