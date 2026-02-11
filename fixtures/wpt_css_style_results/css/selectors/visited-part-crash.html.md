# css/selectors/visited-part-crash.html

```json
{
  "format_version": 3,
  "file": "css/selectors/visited-part-crash.html"
}
```

## style[0]

```css

  .foo:visited::part(test) {}
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
