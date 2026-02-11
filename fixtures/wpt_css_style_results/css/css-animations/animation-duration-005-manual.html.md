# css/css-animations/animation-duration-005-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-duration-005-manual.html"
}
```

## style[0]

```css

  div {
    height: 100px;
    width: 100px;
    position: absolute;
  }

  #test {
    animation-name: sample;
    animation-duration: 0s;
    animation-fill-mode: forwards;

    background-color: blue;
  }

  #ref {
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
