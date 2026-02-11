# css/css-highlight-api/highlight-pseudo-computed.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/highlight-pseudo-computed.html"
}
```

## style[0]

```css

  #target::highlight(foo) {
    background-color: green;
    color: lime;
  }

  #target::highlight(bar) {
    background-color: cyan;
    color: fuchsia;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
