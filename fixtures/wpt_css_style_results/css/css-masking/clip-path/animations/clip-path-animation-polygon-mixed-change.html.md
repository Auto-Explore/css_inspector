# css/css-masking/clip-path/animations/clip-path-animation-polygon-mixed-change.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-polygon-mixed-change.html"
}
```

## style[0]

```css

  .container {
    width: 50px;
    height: 50px;
    background-color: green;
    animation: clippath 20s steps(2, jump-end) -9.999s;
  }

  @keyframes clippath {
    0% {
      clip-path: polygon(0px 0px, 100% 0%, 50% 100%)
    }

    100% {
      clip-path: polygon(20px 20px, 80% 20%, 50% 80%)
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
