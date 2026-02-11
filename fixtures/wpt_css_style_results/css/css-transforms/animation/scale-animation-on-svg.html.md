# css/css-transforms/animation/scale-animation-on-svg.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/scale-animation-on-svg.html"
}
```

## style[0]

```css


@keyframes scale-animation {
    from { scale: 1; }
    to   { scale: 2; }
}

svg {
    width: 400px;
    height: 400px;
}

rect {
    width: 100px;
    height: 100px;
    transform-origin: top left;
    animation: scale-animation 1ms linear forwards;
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
      "message": "Invalid value for property “transform-origin”.",
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
