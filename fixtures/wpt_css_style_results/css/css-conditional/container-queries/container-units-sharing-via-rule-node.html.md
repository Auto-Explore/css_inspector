# css/css-conditional/container-queries/container-units-sharing-via-rule-node.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-units-sharing-via-rule-node.html"
}
```

## style[0]

```css

.test {
  position: relative;
  z-index: 0;
  border: 1px solid;
  max-width: max-content;
}

.container {
  position: absolute;
  container-type: size;
  inset: 0;
}

.container-content {
  position: absolute;
  top: 0;
  left: 0;
  width: 100cqi;
  height: 100cqb;
  background-color: green;
}

span {
  display: block;
  background-color: red;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
