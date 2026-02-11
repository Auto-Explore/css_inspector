# css/css-masking/clip-path/animations/clip-path-animation-ellipse.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-ellipse.html"
}
```

## style[0]

```css

  .container {
    width: 100px;
    height: 100px;
    background-color: green;
    animation: clippath 20s steps(2, jump-end) -9.999s;
  }

  @keyframes clippath {
    0% {
      clip-path: ellipse(60% 30% at 40% 80%);
    }

    100% {
      clip-path: ellipse(10% 20% at 10% 20%);
    }
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
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
