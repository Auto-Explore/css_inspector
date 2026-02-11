# css/css-transforms/animation/translate-animation-on-svg.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/translate-animation-on-svg.html"
}
```

## style[0]

```css


@keyframes translate-animation {
    from { translate: 0 0; }
    to   { translate: 100px 100px; }
}

svg {
    width: 400px;
    height: 400px;
}

rect {
    width: 100px;
    height: 100px;
    transform-origin: top left;
    animation: translate-animation 1ms linear forwards;
}

```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “translate”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “translate”.",
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
