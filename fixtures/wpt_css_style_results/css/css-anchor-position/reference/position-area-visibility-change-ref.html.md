# css/css-anchor-position/reference/position-area-visibility-change-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/reference/position-area-visibility-change-ref.html"
}
```

## style[0]

```css

  .containing-block {
    position: relative;
    width: 150px;
    height: 150px;
    outline: 2px black solid;
  }

  .cell {
    width: 50px;
    height: 50px;
  }

  #anchor-cell {
    position: absolute;
    top: 50px;
    left: 50px;

    background: green;
  }

  .anchor-positioned-cell {
    position: absolute;
  }

  #target-1 {
    top: 0;
    right: 0;
  }

  #target-2 {
    bottom: 0;
    left: 0;
  }

  #target-3 {
    bottom: 0;
    right: 0;
  }

  .blue-background {
    background: blue;
  }

  .magenta-background {
    background: magenta;
  }

  .cyan-background {
    background: cyan;
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
