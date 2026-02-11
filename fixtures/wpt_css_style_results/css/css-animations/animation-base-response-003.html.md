# css/css-animations/animation-base-response-003.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-base-response-003.html"
}
```

## style[0]

```css

  @keyframes font_size_animation {
    from { font-size: 10px; }
    to { font-size: 20px; }
  }
  div {
    font-size: 1px;
    min-width: 1em;
    animation: font_size_animation steps(2, end) 1000s -500s;
  }
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
