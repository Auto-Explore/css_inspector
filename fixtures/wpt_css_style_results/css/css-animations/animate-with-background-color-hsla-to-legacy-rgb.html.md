# css/css-animations/animate-with-background-color-hsla-to-legacy-rgb.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animate-with-background-color-hsla-to-legacy-rgb.html"
}
```

## style[0]

```css

  #target {
    background-color: hsla(from blue h 50% 50% / 100%);
    height: 200px;
    width: 200px;
  }
  @keyframes anim {
    to { background-color: #00f; }
  }
  #target.animated {
    animation: anim 1s both;
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
      "message": "Invalid value for property “background-color”.",
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
