# css/css-animations/animation-keyframes-003-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-keyframes-003-manual.html"
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
    position: relative;
  }

  @keyframes sample {
    from {
      left: 150px;
      animation-timing-function: linear;
    }
    50% {
      left: 75px;
      animation-timing-function: linear;
    }
    to {
      left: 0px;
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
