# css/css-overflow/scroll-markers/scroll-marker-010.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-010.html"
}
```

## style[0]

```css

  #scroller {
    width: 600px;
    height: 300px;
    overflow: scroll;
    scroll-marker-group: after;
    counter-reset: test;
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
    height: 60px;
    display: block;
  }

  #scroller div::scroll-marker {
    counter-increment: test;
    content: counter(test);
    width: 30px;
    height: 30px;
    background-color: blue;
    border-radius: 100%;
    display: inline-block;
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
