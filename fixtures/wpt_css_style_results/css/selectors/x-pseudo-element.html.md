# css/selectors/x-pseudo-element.html

```json
{
  "format_version": 3,
  "file": "css/selectors/x-pseudo-element.html"
}
```

## style[0]

```css

  p {
    color: green;
  }
  ::x-something-nobody-would-think-of, p {
    color: red;
  }
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
