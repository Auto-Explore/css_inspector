# css/css-transforms/animation/transform-percent-with-width-and-height.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/transform-percent-with-width-and-height.html"
}
```

## style[0]

```css


div {
    width: 10px;
    height: 10px;
    background-color: black;
    animation: anim 10s linear forwards;
}

@keyframes anim {
    0.000000001%, to {
        width: 200px;
        height: 200px;
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
