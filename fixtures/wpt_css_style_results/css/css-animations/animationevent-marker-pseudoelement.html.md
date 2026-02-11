# css/css-animations/animationevent-marker-pseudoelement.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animationevent-marker-pseudoelement.html"
}
```

## style[0]

```css

  #target::marker {
    content: "";
    animation: move 1s;
  }

  @keyframes move {
    to { transform: translate(100px); }
  }

  #target {
    display: list-item;
    list-style-position: inside;
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
