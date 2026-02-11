# css/css-syntax/invalid-nested-rules.html

```json
{
  "format_version": 3,
  "file": "css/css-syntax/invalid-nested-rules.html"
}
```

## style[0]

```css

  .a {
    .b <::::invalid::::> {}
    & .c {}
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
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
