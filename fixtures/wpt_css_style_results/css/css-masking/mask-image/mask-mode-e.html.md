# css/css-masking/mask-image/mask-mode-e.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-mode-e.html"
}
```

## style[0]

```css

  body {
    padding: 0;
    margin: 0;
  }

  rect {
    fill: blue;
    width: 100px;
    height: 100px;
    y: 10px;
  }

  rect.auto {
    x: 10px;
    mask-mode: match-source;
    mask-image: linear-gradient(blue 0%, blue 100%);
  }

  rect.alpha {
    x: 120px;
    mask-mode: alpha;
    mask-image: linear-gradient(blue 0%, blue 100%);
  }

  rect.luminance1 {
    x: 230px;
    mask-mode: luminance;
    mask-image: linear-gradient(blue 0%, blue 100%);
  }

  rect.luminance2 {
    x: 340px;
    mask-mode: luminance;
    mask-image: linear-gradient(red 0%, red 100%);
  }

  rect.luminance3 {
    x: 450px;
    mask-mode: luminance;
    mask-image: linear-gradient(lime 0%, lime 100%);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
