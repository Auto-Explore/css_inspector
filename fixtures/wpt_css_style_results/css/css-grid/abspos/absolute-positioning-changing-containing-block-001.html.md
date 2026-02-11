# css/css-grid/abspos/absolute-positioning-changing-containing-block-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/absolute-positioning-changing-containing-block-001.html"
}
```

## style[0]

```css

.wrapper {
  width: 100px;
  height: 100px;
  margin-bottom: 25px;
  background: purple;
  position: relative;
}

.grid {
  display: grid;
  grid-template: 10px / 10px;
  width: 50px;
  height: 50px;
  background: lightblue;
}

.item {
  width: 75%;
  height: 75%;
  background: orange;
  position: absolute;
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
