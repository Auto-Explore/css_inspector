# css/css-break/box-decoration-break-clone-004.html

```json
{
  "format_version": 3,
  "file": "css/css-break/box-decoration-break-clone-004.html"
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
  padding: 5px 0;
  border-block: 10px solid green;
  box-decoration-break: clone;
  block-size: calc(70px + 10px); /* 1st column + 2nd column */
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
