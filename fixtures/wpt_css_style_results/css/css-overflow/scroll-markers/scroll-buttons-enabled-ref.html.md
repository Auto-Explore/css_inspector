# css/css-overflow/scroll-markers/scroll-buttons-enabled-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-buttons-enabled-ref.html"
}
```

## style[0]

```css

  * {
    margin: 0;
    font-family: Ahem;
  }

  #scroller {
    width: 600px;
    height: 300px;
    overflow: auto;
    scroll-marker-group: after;
    white-space: nowrap;
  }

  #scroller div {
    background: green;
    display: inline-block;
    width: 600px;
    height: 270px;
  }

  #scroll-marker-group {
    border: 3px solid black;
    padding: 5px;
    display: flex;
    height: 20px;
    width: 40px;
  }

  #scroll-button-inline-end {
    background: gold;
    display: flex;
    height: 20px;
    width: 20px;
  }

  .scroll-marker {
    width: 10px;
    height: 10px;
    background-color: blue;
    border-radius: 100%;
    display: inline-block;
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
