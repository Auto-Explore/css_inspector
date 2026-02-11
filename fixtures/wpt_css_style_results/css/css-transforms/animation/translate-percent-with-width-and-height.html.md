# css/css-transforms/animation/translate-percent-with-width-and-height.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/translate-percent-with-width-and-height.html"
}
```

## style[0]

```css


div {
    width: 10px;
    height: 10px;
    background-color: black;
    animation: anim 10s linear forwards;
}

@keyframes anim {
    0.000000001%, to {
        width: 200px;
        height: 200px;
        translate: 50%, 50%;
    }
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
      "message": "Invalid value for property “animation”.",
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
