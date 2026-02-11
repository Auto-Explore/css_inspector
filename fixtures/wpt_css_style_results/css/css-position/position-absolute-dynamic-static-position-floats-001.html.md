# css/css-position/position-absolute-dynamic-static-position-floats-001.html

```json
{
  "format_version": 3,
  "file": "css/css-position/position-absolute-dynamic-static-position-floats-001.html"
}
```

## style[0]

```css

#container {
  position: relative;
  background: red;

  box-sizing: border-box;
  border: solid green 10px;
  width: 100px;
  height: 100px;
}

#float {
  float: left;
  background: green;

  width: 40px;
  height: 80px;
}

#target {
  position: absolute;
  background: green;
  display: inline;

  width: 40px;
  height: 80px;
  top: -10px;
  left: -10px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
