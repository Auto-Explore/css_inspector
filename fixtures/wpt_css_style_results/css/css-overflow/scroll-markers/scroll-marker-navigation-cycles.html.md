# css/css-overflow/scroll-markers/scroll-marker-navigation-cycles.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-navigation-cycles.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }

  #scroller {
    overflow: auto;
    width: 100px;
    height: 100px;
    scroll-marker-group: before;
    white-space: nowrap;
  }

  #scroller::scroll-marker-group {
    display: flex;
    height: 10px;
    width: 30px;
  }

  #scroller div {
    background: blue;
    display: inline-block;
    height: 100px;
    width: 100px;
  }

  #scroller div::scroll-marker {
    content: "";
    background: blue;
    width: 10px;
    height: 10px;
  }

  #scroller div::scroll-marker:target-current {
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
