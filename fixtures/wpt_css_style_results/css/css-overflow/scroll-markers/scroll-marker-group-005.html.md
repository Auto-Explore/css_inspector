# css/css-overflow/scroll-markers/scroll-marker-group-005.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-005.html"
}
```

## style[0]

```css

  div.scroll-marker-group-before {
    overflow: auto;
    scroll-marker-group: before;
  }

  div.scroll-marker-group-after {
    overflow: auto;
    scroll-marker-group: after;
  }

  div::before {
    background: red;
    content: "";
    border-radius: 50%;
    display: flex;
    height: 30px;
    width: 30px;
  }

  div::after {
    background: yellow;
    content: "";
    border-radius: 50%;
    display: flex;
    height: 30px;
    width: 30px;
  }

  div::scroll-marker-group {
    background: green;
    border-radius: 50%;
    display: flex;
    height: 30px;
    width: 30px;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “scroll-marker-group”.",
      "severity": "Error"
    },
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
