# css/css-animations/cancel-animation-shadow-slot-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/cancel-animation-shadow-slot-invalidation.html"
}
```

## style[0]

```css

  @keyframes anim {
    from { color: red; }
    to { color: green; }
  }
  #anim { animation: anim 100s; }
  .none { display: none; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
