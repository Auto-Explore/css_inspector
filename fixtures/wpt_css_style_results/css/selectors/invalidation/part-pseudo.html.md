# css/selectors/invalidation/part-pseudo.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/part-pseudo.html"
}
```

## style[0]

```css
#host::part(green) { color: red; }
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
