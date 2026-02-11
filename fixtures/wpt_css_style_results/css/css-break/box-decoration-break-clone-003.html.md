# css/css-break/box-decoration-break-clone-003.html

```json
{
  "format_version": 3,
  "file": "css/css-break/box-decoration-break-clone-003.html"
}
```

## style[0]

```css

.multicol {
  column-count: 2;
  column-gap: 0;
  column-fill: auto;
  inline-size: 100px;
  block-size: 100px;
  background: red;
}

.container {
  border-block-start: 15px solid green;
  border-block-end: 15px solid pink;
  box-decoration-break: clone;
  block-size: 70px;
}

.child {
  block-size: calc(85px + 100px); /* 1st column + 2nd column */
  background: green;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border-block-start”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-block-end”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
