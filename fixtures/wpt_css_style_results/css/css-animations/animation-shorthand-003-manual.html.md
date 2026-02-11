# css/css-animations/animation-shorthand-003-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-shorthand-003-manual.html"
}
```

## style[0]

```css

  div::before {
    animation: 10s 2s sample;

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
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
