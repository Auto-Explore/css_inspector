# css/css-overflow/scroll-markers/scroll-marker-006.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-006.html"
}
```

## style[0]

```css

  .container {
    scroll-marker-group: before;
    overflow: hidden;
  }

  .container::scroll-marker-group {
    display: flex;
    height: 25px;
  }

  .marker::scroll-marker {
    display: block;
    width: 25px;
    height: 25px;
    content: "";
    background: green;
  }

  .contain_abs {
    position: relative;
  }

  .contain_all {
    contain: layout;
  }

  .uncontained::scroll-marker {
    z-index: 100;
    content: "ERROR";
    background: red;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
