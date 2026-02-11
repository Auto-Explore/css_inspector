# css/selectors/is-where-error-crash.html

```json
{
  "format_version": 3,
  "file": "css/selectors/is-where-error-crash.html"
}
```

## style[0]

```css

:is(c|l:i) {}
:where(c|l:i) {}
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
