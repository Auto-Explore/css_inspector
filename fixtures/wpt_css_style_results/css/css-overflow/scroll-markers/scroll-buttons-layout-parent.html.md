# css/css-overflow/scroll-markers/scroll-buttons-layout-parent.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-buttons-layout-parent.html"
}
```

## style[0]

```css

  #sc {
    z-index: 1;
    overflow: hidden;
    width: 100px;
    height: 100px;
    background: red;
  }
  #sc::scroll-button(left) {
    content: "";
    border: none;
    margin-left: -100px;
    z-index: 2;
    width: 50px;
    height: 100px;
    background: green;
  }
  #sc::scroll-button(right) {
    content: "";
    border: none;
    z-index: 2;
    width: 50px;
    height: 100px;
    background: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
