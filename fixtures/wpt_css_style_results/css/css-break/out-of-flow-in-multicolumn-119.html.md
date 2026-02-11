# css/css-break/out-of-flow-in-multicolumn-119.html

```json
{
  "format_version": 3,
  "file": "css/css-break/out-of-flow-in-multicolumn-119.html"
}
```

## style[0]

```css

.multicol {
  writing-mode: vertical-rl;
  block-size: 100px;
  inline-size: 100px;
  column-count: 2;
  column-fill: auto;
  column-gap: 0;
  background: red;
}
.container {
  display: flex;
  position: relative;
  border-block: 20px solid green;
  block-size: 160px;
}

.abspos {
  position: absolute;
  inline-size: 100%;
  block-size: 160px;
  background: green;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-block”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
