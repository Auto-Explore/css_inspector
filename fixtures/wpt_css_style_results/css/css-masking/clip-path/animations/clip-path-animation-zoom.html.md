# css/css-masking/clip-path/animations/clip-path-animation-zoom.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-zoom.html"
}
```

## style[0]

```css

  .container {
    width: 80px;
    height: 80px;
    background-color: green;
    zoom: 1.25;
    animation: clippath 20s steps(2, jump-end) -9.999s;
  }

  @keyframes clippath {
    0% {
      clip-path: circle(50% at 50% 50%);
    }

    100% {
      clip-path: circle(20% at 20% 20%);
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
