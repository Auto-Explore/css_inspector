# css/css-overflow/scroll-markers/html-scroll-marker-target-before-after.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/html-scroll-marker-target-before-after.html"
}
```

## style[0]

```css

  .scroller {
    width: 600px;
    height: 300px;
    overflow: scroll;
  }

  .scroller div {
    width: 600px;
    height: 300px;
    margin-bottom: 20px;
    background: green;
  }

  .wrapper {
    scroll-target-group: auto;
  }

  .wrapper a {
    color: blue;
  }

  .wrapper a:target-current {
    color: green;
  }

  .wrapper a:target-before {
    color: red;
  }

  .wrapper a:target-after {
    color: yellow;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
