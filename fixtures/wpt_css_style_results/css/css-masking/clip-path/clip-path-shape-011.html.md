# css/css-masking/clip-path/clip-path-shape-011.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-shape-011.html"
}
```

## style[0]

```css

#shape {
  width: 400px;
  height: 300px;
  background: green;
  clip-path: shape(
    from 0px 100px,
    arc to 20px 100px of 10% large cw,
    arc to 100px 20px of 5% small,
    arc to 0 100px of calc(10px + 15%)
    rotate 30deg);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
