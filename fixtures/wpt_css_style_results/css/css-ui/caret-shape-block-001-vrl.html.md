# css/css-ui/caret-shape-block-001-vrl.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/caret-shape-block-001-vrl.html"
}
```

## style[0]

```css

  #wrapper {
    writing-mode: vertical-rl;
    width: stretch;
  }
  #target {
    font-size: 3em;
    caret-color: lime;
    caret-shape: block;
    caret-animation: manual;
  }
  #target:focus {
    outline: none;
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
