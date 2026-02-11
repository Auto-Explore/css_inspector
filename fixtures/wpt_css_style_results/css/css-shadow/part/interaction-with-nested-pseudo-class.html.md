# css/css-shadow/part/interaction-with-nested-pseudo-class.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/interaction-with-nested-pseudo-class.html"
}
```

## style[0]

```css

  :root { color: red; }
  ::part(test):is(:focus) { color: green; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
