# css/css-overflow/scroll-markers/scroll-button-elementFromPoint.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-button-elementFromPoint.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }

  #scroller {
    margin-top: 100px;
    width: 600px;
    height: 300px;
    overflow: auto;
    white-space: nowrap;
  }

  #scroller div {
    display: inline-block;
    width: 600px;
    height: 200px;
  }

  #scroller::scroll-button(left) {
    position: absolute;
    top: 0;
    left: 0;
    content: "";
    width: 100px;
    height: 20px;
    display: inline-block;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
