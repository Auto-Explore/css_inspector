# css/css-overflow/scroll-markers/scroll-button-activation-content-hit-test.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-button-activation-content-hit-test.html"
}
```

## style[0]

```css

  * {
    margin: 0;
    padding: 0;
  }

  #scroller {
    width: 600px;
    height: 300px;
    overflow: auto;
    white-space: nowrap;
  }

  #scroller div {
    background: green;
    display: inline-block;
    width: 600px;
    height: 270px;
  }

  #scroller :first-child {
    background: purple;
  }

  #scroller::scroll-button(inline-end) {
    content: ">";
    font-family: Ahem;
    background: blue;
    height: 20px;
    width: 20px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
