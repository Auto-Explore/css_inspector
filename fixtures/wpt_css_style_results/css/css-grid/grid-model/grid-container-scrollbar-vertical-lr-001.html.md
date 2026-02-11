# css/css-grid/grid-model/grid-container-scrollbar-vertical-lr-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-model/grid-container-scrollbar-vertical-lr-001.html"
}
```

## style[0]

```css

  .grid {
    font: 10px/1 Ahem;
    margin: 10px;
    writing-mode: vertical-lr;
  }

  .scrollX {
    overflow-x: scroll;
  }

  .scrollY {
    overflow-y: scroll;
  }

  .fixedSize {
    width: 200px;
    height: 50px;
  }

  .grid > div {
    background: cyan;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
