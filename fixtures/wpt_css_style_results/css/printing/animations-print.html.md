# css/printing/animations-print.html

```json
{
  "format_version": 3,
  "file": "css/printing/animations-print.html"
}
```

## style[0]

```css


@keyframes a {
  from, to { color: blue }
}

p {
  color: olive;
  animation: a 1s infinite;
}

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
