# css/css-transforms/animation/translate-animation-on-svg-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/translate-animation-on-svg-ref.html"
}
```

## style[0]

```css


svg {
    width: 400px;
    height: 400px;
}

rect {
    width: 200px;
    height: 200px;
    transform-origin: top left;
    translate: 100px 100px;
}

```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “translate”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
