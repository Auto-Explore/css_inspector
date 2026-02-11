# css/css-animations/crashtests/cancel-update.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/crashtests/cancel-update.html"
}
```

## style[0]

```css

  @keyframes anim {
    from { background-color: blue; }
    to { background-color: red; }
  }

  @keyframes anim2 {
    from { opacity: 0; }
    to { opacity: 1; }
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
