# css/css-backgrounds/background-clip/clip-text-out-of-flow-child.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-clip/clip-text-out-of-flow-child.html"
}
```

## style[0]

```css

  .box {
    font: 10px/1 Ahem;
    width: 100px;
    height: 100px;
    background-color: aqua;
    color: transparent;
    background-clip: text;
    position: relative;

    &::before {
      width: 100px;
      height: 100px;
      background-color: blueviolet;
      content: '_';
      position: absolute;
      z-index: -1;
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
