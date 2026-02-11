# css/css-overflow/scroll-markers/scroll-button-on-object.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-button-on-object.html"
}
```

## style[0]

```css

  #target {
    height: 0;
    width: 0;
  }

  #target::scroll-button(block-end) {
    content: "scroll-button";
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
