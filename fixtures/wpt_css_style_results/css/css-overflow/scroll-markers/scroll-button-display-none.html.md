# css/css-overflow/scroll-markers/scroll-button-display-none.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-button-display-none.html"
}
```

## style[0]

```css

  #scroller::scroll-button(left) {
    content: "";
    background-color: red;
    height: 100px;
    width: 100px;
    top: 0;
    left: 0;
    position: absolute;
  }

  #scroller.hide::scroll-button(left) {
    display: none;
  }

  #scroller {
    width: 300px;
    height: 300px;
    overflow: auto;
  }

  #filler {
    height: 20000x;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
