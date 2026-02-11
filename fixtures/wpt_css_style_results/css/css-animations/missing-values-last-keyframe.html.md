# css/css-animations/missing-values-last-keyframe.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/missing-values-last-keyframe.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }

  .box {
    position: relative;
    width: 100px;
    height: 100px;
    left: 0;
    background-color: green;
  }

  #box1 {
    left: 200px;
    animation: move-left 10s linear;
  }

  #box2 {
    transform: translateX(200px);
    animation: move-transform 10s linear;
  }

  @keyframes move-left {
    0% {
      left: 0;
      opacity: 0;
    }
    50% {
      left: 0;
      opacity: 1;
    }
    75% {
      opacity: 1;
    }
    100% {
      opacity: 1;
    }
  }

  @keyframes move-transform {
    0% {
      transform: translateX(0);
      opacity: 0;
    }
    50% {
      transform: translateX(0);
      opacity: 1;
    }
    75% {
      opacity: 1;
    }
    100% {
      opacity: 1;
    }
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
