# css/css-masking/mask-image/mask-clip-5.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-clip-5.html"
}
```

## style[0]

```css

  svg {
    position: absolute;
    top: 10px;
    border: 1px solid black;
  }

  foreignObject.mask {
    mask-origin: fill-box;
    mask-repeat: no-repeat;
    mask-image: url(support/50x50-opaque-blue.svg);
  }

  foreignObject.mask > div.content {
    background-color: blue;
    width: 100%;
    height: 100%;
  }

  div.border {
    box-sizing: border-box;
    border: 10px solid green;
  }

  foreignObject.view {
    mask-clip: view-box;
  }

  foreignObject.fill {
    mask-clip: fill-box;
  }

  foreignObject.stroke {
    mask-clip: stroke-box;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
