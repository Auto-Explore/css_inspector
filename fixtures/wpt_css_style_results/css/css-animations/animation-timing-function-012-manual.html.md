# css/css-animations/animation-timing-function-012-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-timing-function-012-manual.html"
}
```

## style[0]

```css

  div::before {
    animation-name: sample;
    animation-duration: 10s;
    animation-timing-function: linear;

    background-color: blue;
    content: "Filler Text";
    display: block;
    height: 100px;
    width: 100px;
    position: relative;
  }

  @keyframes sample {
    from {
      left: 150px;
    }
    to {
      left: 0px;
    }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
