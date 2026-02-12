# css/css-transforms/animation/transform-non-invertible-discrete-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/transform-non-invertible-discrete-interpolation.html"
}
```

## style[0]

```css


div {
    width: 100px;
    height: 100px;
    background-color: black;
    animation: anim 100s linear forwards;
}

@keyframes anim {
    from { transform: matrix3d(2,0,0,0, 0,2,0,0, 0,0,0,0, 0,0,0,1) }
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
