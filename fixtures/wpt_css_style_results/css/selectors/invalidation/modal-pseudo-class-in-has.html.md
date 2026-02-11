# css/selectors/invalidation/modal-pseudo-class-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/modal-pseudo-class-in-has.html"
}
```

## style[0]

```css

  #subject:has(#dialog:modal) { color: green; }
  #subject:has(#fullscreenTarget:modal) { color: blue; }
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
