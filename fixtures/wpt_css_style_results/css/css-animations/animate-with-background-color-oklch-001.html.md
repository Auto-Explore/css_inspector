# css/css-animations/animate-with-background-color-oklch-001.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animate-with-background-color-oklch-001.html"
}
```

## style[0]

```css

@keyframes bg-color-oklch {
  to {
    background-color: oklch(45% 0.2 264); /*  blue */
  }
}
#box {
  block-size: 200px;
  inline-size: 200px;
  background-color: #ff0000;
  animation: linear 1s forwards bg-color-oklch;
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
