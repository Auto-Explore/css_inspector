# css/css-animations/animation-timing-function-007-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-timing-function-007-manual.html"
}
```

## style[0]

```css

  div {
    height: 100px;
    width: 100px;
    position: absolute;
  }

  #test-step-start {
    animation-name: sample;
    animation-duration: 10s;
    animation-fill-mode: forwards;
    animation-timing-function: step-start;

    background-color: blue;
  }

  #ref-no-animation {
    background-color: red;
    left: 150px;
  }

  @keyframes sample {
    from {
      left: 0px;
    }
    to {
      left: 150px;
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
