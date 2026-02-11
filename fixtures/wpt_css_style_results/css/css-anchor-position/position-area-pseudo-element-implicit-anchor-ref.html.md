# css/css-anchor-position/position-area-pseudo-element-implicit-anchor-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-pseudo-element-implicit-anchor-ref.html"
}
```

## style[0]

```css

body { margin: 0 }
#target  {
    margin-top: 100px;
    margin-left: 50px;
    width: 100px;
    height: 100px;
    background: blue;
    position: relative;
}
#target > div {
    position: absolute;
    width: 100px;
    height: 100px;
    background: green;
}
#bottom {
    top: 100px;
    left: 0px;
}
#right {
    top: 0px;
    left: 100px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
