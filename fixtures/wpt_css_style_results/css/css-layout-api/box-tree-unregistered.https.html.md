# css/css-layout-api/box-tree-unregistered.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/box-tree-unregistered.https.html"
}
```

## style[0]

```css

@supports (display: layout(unregistered)) {
  .test {
    display: layout(unregistered);
  }
}

.container {
  float: left;
  margin: 20px 0;
  border: solid 2px;
  width: 100px;
}

.float {
  float: left;
  width: 100%;
  height: 40px;
  background: hotpink;
}

.inflow {
  height: 40px;
  background: hotpink;
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
