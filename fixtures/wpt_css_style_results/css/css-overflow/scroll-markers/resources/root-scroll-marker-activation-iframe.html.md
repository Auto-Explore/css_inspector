# css/css-overflow/scroll-markers/resources/root-scroll-marker-activation-iframe.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/resources/root-scroll-marker-activation-iframe.html"
}
```

## style[0]

```css

  :root {
    scroll-marker-group: before;
  }

  .item {
    height: 100vh;
  }

  .item::scroll-marker {
    content: '';
    background-color: blue;
    width: 20px;
    height: 20px;
    display: inline-block;
    margin-left: 10px;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “scroll-marker-group”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
