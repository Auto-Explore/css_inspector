# css/css-layout-api/box-tree-registered.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/box-tree-registered.https.html"
}
```

## style[0]

```css

@supports (display: layout(registered)) {
  .test {
    display: layout(registered);
  }
}

.container {
  margin: 20px 0;
  border: solid 2px;
  width: 100px;
}

.float {
  float: left;
  width: 50%;
  height: 50px;
}

.pink {
  background: hotpink;
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
