# css/css-overflow/scroll-markers/scroll-buttons-002.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-buttons-002.html"
}
```

## style[0]

```css

  :root {
    --scroll-buttons-background: red;
  }

  div::scroll-button(inline-start) {
    content: "";
    background: var(--scroll-buttons-background);
    display: flex;
    height: 50px;
    width: 100px;
    border: none;
  }

  div::scroll-button(down) {
    content: "";
    background: var(--scroll-buttons-background);
    display: flex;
    height: 50px;
    width: 100px;
    border: none;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
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
