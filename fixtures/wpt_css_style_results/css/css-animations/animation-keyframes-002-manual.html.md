# css/css-animations/animation-keyframes-002-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-keyframes-002-manual.html"
}
```

## style[0]

```css

  div {
    animation-name: sample;
    animation-duration: 10s;

    background-color: blue;
    height: 100px;
    width: 100px;
  }

  @keyframes sample {
    -100% {
      background-color: red;
    }
    0% {
      background-color: blue;
    }
    30% {
      background-color: green;
    }
    65% {
      background-color: yellow;
    }
    100% {
      background-color: blue;
    }
    200% {
      background-color: red;
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
