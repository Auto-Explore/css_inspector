# css/css-grid/grid-model/grid-container-scrollbar-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-model/grid-container-scrollbar-001-ref.html"
}
```

## style[0]

```css

  .container {
    font: 10px/1 Ahem;
    margin: 10px;
    background: grey;
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

  .container > div {
    background: cyan;
    width: 100%;
    height: 100%;
  }

  .directionRTL {
    direction: rtl;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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
