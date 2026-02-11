# css/css-grid/grid-model/grid-box-sizing-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-model/grid-box-sizing-001.html"
}
```

## style[0]

```css

.wrapper {
  position: relative;
  width: 200px;
  height: 100px;
}

.grid {
  position: absolute;
  left: 0;
  top: 0;
  display: grid;
  border-style: solid;
  border-width: 5px 10px 15px 20px;
  padding: 17px 13px 7px 3px;
}

.wholeWidth {
  right: 0;
}

.wholeHeight {
  bottom: 0;
}
.item {
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
