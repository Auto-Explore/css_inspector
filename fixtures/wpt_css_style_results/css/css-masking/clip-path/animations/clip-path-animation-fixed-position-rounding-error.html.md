# css/css-masking/clip-path/animations/clip-path-animation-fixed-position-rounding-error.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-fixed-position-rounding-error.html"
}
```

## style[0]

```css

  .container {
    background-color: green;
    animation: clippath 20s steps(2, jump-end) -9.999s;
    position: fixed;
    width: 70px;
    height: 126px;
  }

  @keyframes clippath {
    0% {
      clip-path: inset(0% 0%);
    }

    100% {
      clip-path: inset(10% 10%);
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
