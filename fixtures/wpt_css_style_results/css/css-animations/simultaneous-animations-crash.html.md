# css/css-animations/simultaneous-animations-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/simultaneous-animations-crash.html"
}
```

## style[0]

```css

  @keyframes animationKeyframes {
    from { opacity: 0.1; filter: blur(1px); }
    to { opacity: 0.9; filter: blur(5px); }
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
