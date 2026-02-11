# css/css-animations/keyframes-remove-documentElement-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/keyframes-remove-documentElement-crash.html"
}
```

## style[0]

```css

  @keyframes anim {
    from { color: pink }
    to { color: purple }
  }
  div {
    animation: notfound 1s;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
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
