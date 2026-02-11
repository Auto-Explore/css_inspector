# css/css-animations/animation-timing-function-010-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-timing-function-010-manual.html"
}
```

## style[0]

```css

  div {
    float: left;
    height: 100px;
    width: 100px;
    position: absolute;
  }

  #test-step-end {
    animation-name: sample;
    animation-duration: 10s;
    animation-fill-mode: forwards;
    animation-timing-function: steps(3, end);

    background-color: blue;
  }

  #ref-1 {
    background-color: yellow;
    left: 200px;
  }

  #ref-2 {
    background-color: green;
    left: 100px;
  }

  #ref-3 {
    background-color: red;
    left: 0px;
  }

  @keyframes sample {
    from {
      left: 300px;
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
