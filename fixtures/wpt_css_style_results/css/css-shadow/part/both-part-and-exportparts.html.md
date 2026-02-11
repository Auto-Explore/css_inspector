# css/css-shadow/part/both-part-and-exportparts.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/both-part-and-exportparts.html"
}
```

## style[0]

```css

  ::part(bar) {
    color: green;
  }
  ::part(baz) {
    background: lime;
  }
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
