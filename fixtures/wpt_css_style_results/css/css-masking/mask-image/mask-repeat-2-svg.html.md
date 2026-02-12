# css/css-masking/mask-image/mask-repeat-2-svg.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-repeat-2-svg.html"
}
```

## style[0]

```css

  rect.frame {
    x: -0.5px;
    y: -0.5px;
    width: 129px;
    height: 129px;
    stroke: black;
    fill: none;
  }

  rect.masked {
    width: 128px;
    height: 128px;
    fill: purple;
    mask-image: url(support/50x50-opaque-blue.svg);
    mask-position: left top;
  }

  #space {
    mask-repeat: space;
  }
  #space-x {
    mask-repeat: space no-repeat;
  }
  #space-y {
    mask-repeat: no-repeat space;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
