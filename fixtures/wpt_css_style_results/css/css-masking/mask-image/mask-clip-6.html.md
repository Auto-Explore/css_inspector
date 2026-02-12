# css/css-masking/mask-image/mask-clip-6.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-clip-6.html"
}
```

## style[0]

```css

  foreignObject.mask {
    mask-origin: view-box;
    mask-repeat: no-repeat;
    mask-image: url(support/50x50-opaque-blue.svg);
    mask-size: 100px 100px;
    mask-clip: no-clip;
    overflow: visible;
  }

  foreignObject.mask > div.content {
    background-color: green;
    position: absolute;
    left: -50px;
    top: -20px;
    width: 150px;
    height: 150px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
