# css/css-ui/caret-color-021.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/caret-color-021.html"
}
```

## style[0]

```css

  @keyframes caret-color-to-lime {
    to { caret-color: lime; }
  }

  #textarea {
    color: magenta;
    animation: caret-color-to-lime 2s -1s paused;
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
