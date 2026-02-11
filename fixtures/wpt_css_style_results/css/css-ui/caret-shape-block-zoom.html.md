# css/css-ui/caret-shape-block-zoom.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/caret-shape-block-zoom.html"
}
```

## style[0]

```css

  div {
    font-size: 3em;
    zoom: 2;
  }

  #target {
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
