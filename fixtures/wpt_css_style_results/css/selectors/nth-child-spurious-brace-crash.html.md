# css/selectors/nth-child-spurious-brace-crash.html

```json
{
  "format_version": 3,
  "file": "css/selectors/nth-child-spurious-brace-crash.html"
}
```

## style[0]

```css

s:nth-child(n-{
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
