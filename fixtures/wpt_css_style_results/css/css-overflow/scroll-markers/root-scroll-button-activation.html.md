# css/css-overflow/scroll-markers/root-scroll-button-activation.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/root-scroll-button-activation.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }

  :root::scroll-button(block-end) {
    content: "down";
    position: absolute;
    top: 0;
    width: 30px;
    height: 30px;;
  }

  div {
    width: 600px;
    height: 300px;
    margin-bottom: 20px;
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
