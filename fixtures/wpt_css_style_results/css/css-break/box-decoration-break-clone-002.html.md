# css/css-break/box-decoration-break-clone-002.html

```json
{
  "format_version": 3,
  "file": "css/css-break/box-decoration-break-clone-002.html"
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
  border-block: 15px solid green;
  box-decoration-break: clone;
  block-size: calc(70px + 30px); /* 1st column + 2nd column */
}

.child {
  block-size: calc(70px + 85px); /* 1st column + 2nd column */
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
