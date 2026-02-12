# css/css-overflow/scroll-markers/scroll-marker-next-focus.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-next-focus.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }

  #scroller {
    overflow: auto;
    scroll-marker-group: before;
  }

  #scroller::scroll-marker-group {
    height: 10px;
    width: 150px;
  }

  section {
    background: red;
  }

  section:focus {
    background: green;
  }

  section::scroll-marker {
    content: "Section";
    background: red;
    width: 50px;
    height: 10px;
  }

  section::scroll-marker:focus {
    background: green;
  }

  button {
    background: red;
    height: 30px;
  }

  button:focus {
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
