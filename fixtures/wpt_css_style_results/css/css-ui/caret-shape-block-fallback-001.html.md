# css/css-ui/caret-shape-block-fallback-001.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/caret-shape-block-fallback-001.html"
}
```

## style[0]

```css

  #target {
    font: 100px/1 Ahem;
    color: green;
    caret-color: green;
    caret-shape: block;
    caret-animation: manual;
    width: 2ch;
    background: red;
  }
  #target:focus {
    outline: none;
  }
  svg {
    vertical-align: top;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “caret-animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
