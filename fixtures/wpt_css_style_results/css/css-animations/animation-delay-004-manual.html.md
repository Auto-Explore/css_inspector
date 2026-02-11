# css/css-animations/animation-delay-004-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-delay-004-manual.html"
}
```

## style[0]

```css

  div::after {
    animation-name: sample;
    animation-duration: 5s;
    animation-delay: 5s;

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
