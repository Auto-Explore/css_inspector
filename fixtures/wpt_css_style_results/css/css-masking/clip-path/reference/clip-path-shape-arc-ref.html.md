# css/css-masking/clip-path/reference/clip-path-shape-arc-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/reference/clip-path-shape-arc-ref.html"
}
```

## style[0]

```css

#shape {
  /* 500/sqrt(2) ~= 353 */
  width: 353px;
  height: 353px;
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
