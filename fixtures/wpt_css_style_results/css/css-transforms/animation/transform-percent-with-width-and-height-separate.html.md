# css/css-transforms/animation/transform-percent-with-width-and-height-separate.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/transform-percent-with-width-and-height-separate.html"
}
```

## style[0]

```css


div {
    width: 10px;
    height: 10px;
    background-color: black;
    animation-duration: 10s;
    animation-name: size, transform;
}

@keyframes size {
    0.000000001%, to {
        width: 200px;
        height: 200px;
    }
}

@keyframes transform {
    0.000000001%, to {
        transform: translate(50%, 50%);
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
