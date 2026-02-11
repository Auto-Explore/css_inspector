# css/css-animations/display-none-prevents-starting-in-subtree.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/display-none-prevents-starting-in-subtree.html"
}
```

## style[0]

```css

@keyframes margin {
    100% { margin-left: 200px }
}

#child {
  animation: margin 1s forwards;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
