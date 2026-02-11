# css/css-transitions/starting-style-first-letter-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/starting-style-first-letter-crash.html"
}
```

## style[0]

```css

@starting-style {
  div::first-letter {
    color: red;
  }
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
