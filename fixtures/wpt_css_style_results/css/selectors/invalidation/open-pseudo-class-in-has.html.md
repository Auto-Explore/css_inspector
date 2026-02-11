# css/selectors/invalidation/open-pseudo-class-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/open-pseudo-class-in-has.html"
}
```

## style[0]

```css

  #subject:has(#dialog:open) { color: green; }
  #subject:has(#details:open) { color: blue; }
  #subject:has(#select:open) { color: red; }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
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
