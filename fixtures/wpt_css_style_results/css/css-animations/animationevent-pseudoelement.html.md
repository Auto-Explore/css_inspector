# css/css-animations/animationevent-pseudoelement.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animationevent-pseudoelement.html"
}
```

## style[0]

```css

  #target::before {
    content: "";
    animation: move 1s;
  }

  @keyframes move {
    to { transform: translate(100px); }
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
