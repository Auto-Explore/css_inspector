# css/css-overflow/scroll-markers/scroll-marker-004.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-004.html"
}
```

## style[0]

```css

  #scroller {
    width: 600px;
    height: 300px;
    overflow: scroll;
    scroll-marker-group: after;
  }

  #scroller div {
    width: 600px;
    height: 300px;
    margin-bottom: 20px;
    background: green;
  }

  #scroller::scroll-marker-group {
    border: 3px solid black;
    padding: 5px;
    display: flex;
    height: 40px;
  }

  #scroller div::scroll-marker {
    content: "";
    width: 10px;
    height: 10px;
    background-color: blue;
    border-radius: 100%;
  }

  #scroller #first::scroll-marker {
    background-color: purple;
    margin-right: 4px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
