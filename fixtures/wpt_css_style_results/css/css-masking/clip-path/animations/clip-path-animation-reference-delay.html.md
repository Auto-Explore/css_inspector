# css/css-masking/clip-path/animations/clip-path-animation-reference-delay.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-reference-delay.html"
}
```

## style[0]

```css

  .container {
    width: 100px;
    height: 100px;
    background-color: green;
    animation: clippath 1s 10s;
    animation-fill-mode: none;
    clip-path: url(#path);
    position: absolute;
    left: 10px;
    top: 10px;
  }

  .child {
    width: 10px;
    height: 10px;
    background-color: blue;
    left: 150px;
    position: absolute;
  }

  @keyframes clippath {
    0% {
      clip-path: circle(50% at 50% 50%);
    }

    100% {
      clip-path: circle(35% at 35% 35%);
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
