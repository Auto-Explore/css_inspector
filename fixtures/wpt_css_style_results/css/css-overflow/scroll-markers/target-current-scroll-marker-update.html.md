# css/css-overflow/scroll-markers/target-current-scroll-marker-update.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/target-current-scroll-marker-update.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }

  #scroller {
    width: 600px;
    height: 300px;
    overflow: auto;
    scroll-marker-group: before;
    white-space: nowrap;
  }

  #scroller div {
    background: pink;
    display: inline-block;
    width: 600px;
    height: 270px;
  }

  #scroller::scroll-marker-group {
    display: flex;
    height: 20px;
    width: 40px;
  }

  #scroller div::scroll-marker {
    content: "";
    width: 20px;
    height: 20px;
    background-color: green;
    display: inline-block;
  }

  #scroller div::scroll-marker:target-current {
    background-color: blue;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
