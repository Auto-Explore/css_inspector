# css/css-animations/CSSAnimation-startTime.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/CSSAnimation-startTime.tentative.html"
}
```

## style[0]

```css


.animated-div {
  margin-left: 10px;
  /* Make it easier to calculate expected values: */
  animation-timing-function: linear ! important;
}

@keyframes anim {
  from { margin-left: 100px; }
  to { margin-left: 200px; }
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
      "message": "Invalid value for property “animation-timing-function”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
