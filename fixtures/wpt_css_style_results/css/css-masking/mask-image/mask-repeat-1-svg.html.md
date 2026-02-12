# css/css-masking/mask-image/mask-repeat-1-svg.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-repeat-1-svg.html"
}
```

## style[0]

```css

  svg {
    display: block;
  }

  rect.frame {
    x: -0.5px;
    y: -0.5px;
    width: 116px;
    height: 116px;
    stroke: black;
    fill: none;
  }

  rect.masked {
    width: 115px;
    height: 115px;
    fill: purple;
    mask-image: url(support/50x50-opaque-blue.svg);
    mask-position: left top;
  }

  #no-repeat {
    mask-repeat: no-repeat no-repeat;
  }
  #repeat {
    mask-repeat: repeat repeat;
  }
  #repeat-x {
    mask-repeat: repeat no-repeat;
  }
  #repeat-y {
    mask-repeat: no-repeat repeat;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
