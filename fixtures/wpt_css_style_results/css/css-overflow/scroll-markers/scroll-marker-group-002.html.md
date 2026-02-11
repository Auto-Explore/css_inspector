# css/css-overflow/scroll-markers/scroll-marker-group-002.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-002.html"
}
```

## style[0]

```css

  :root {
    --scroll-marker-group-background: red;
  }

  div {
    overflow: auto;
    scroll-marker-group: before;
  }

  div::scroll-marker-group {
    background: var(--scroll-marker-group-background);
    display: flex;
    height: 100px;
    width: 100px;
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
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
