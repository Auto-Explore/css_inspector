# css/css-overflow/scroll-markers/scroll-marker-013.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-013.html"
}
```

## style[0]

```css

  #container {
    font: 20px/20px Ahem;
    overflow: auto;
    width: 300px;
    height: 100px;
    scroll-marker-group: before;
    background: yellow;
  }

  #container::scroll-marker-group {
    display: block;
    height: 20px;
    background: cyan;
  }

  #container>* {
    height: 100px;
  }

  #container>*::scroll-marker {
    content: "x";
  }
```

```json
{
  "errors": 4,
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
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
