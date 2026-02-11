# css/css-masking/clip/clip-no-stacking-context.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip/clip-no-stacking-context.html"
}
```

## style[0]

```css

  #bottom {
    position: absolute;
    width: 100px;
    height: 100px;
    background: red;
    z-index: 1;
  }
  #clip {
    position: absolute;
    clip: rect(0px, 100px, 100px, 0px);
  }
  #top {
    position: absolute;
    width: 100px;
    height: 100px;
    background: green;
    z-index: 3;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
