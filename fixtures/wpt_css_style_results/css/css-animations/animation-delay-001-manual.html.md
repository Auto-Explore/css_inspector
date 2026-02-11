# css/css-animations/animation-delay-001-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-delay-001-manual.html"
}
```

## style[0]

```css

  div {
    animation-timing-function: linear;

    height: 100px;
    width: 100px;
    position: relative;
  }

  #test-negative-delay {
    animation-name: test-negative-delay;
    animation-duration: 10s;
    animation-delay: -5s;

    background-color: blue;
  }

  #ref-no-delay {
    animation-name: ref-no-delay;
    animation-duration: 5s;

    background-color: yellow;
  }

  @keyframes test-negative-delay {
    from {
      left: 150px;
    }
    to {
      left: 0px;
    }
  }

  @keyframes ref-no-delay {
    from {
      left: 75px;
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
