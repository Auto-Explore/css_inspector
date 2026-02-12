# css/css-break/box-decoration-break-clone-001.html

```json
{
  "format_version": 3,
  "file": "css/css-break/box-decoration-break-clone-001.html"
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
  background: green;
}

.child {
  block-size: calc(70px + 70px); /* 1st column + 2nd column */
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
