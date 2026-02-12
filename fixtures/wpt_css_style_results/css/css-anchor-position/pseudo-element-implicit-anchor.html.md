# css/css-anchor-position/pseudo-element-implicit-anchor.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/pseudo-element-implicit-anchor.html"
}
```

## style[0]

```css

  #anchor::after {
    content: "";
    position: absolute;
    position-anchor: auto;
    width: 100px;
    height: 100px;
    bottom: anchor(top);
    background: green;
  }
  #ref {
    width: 100px;
    height: 100px;
    background: red;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
