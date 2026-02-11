# css/css-position/position-absolute-dynamic-static-position-margin-001.html

```json
{
  "format_version": 3,
  "file": "css/css-position/position-absolute-dynamic-static-position-margin-001.html"
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

#block {
  background: green;
  height: 40px;
  margin-bottom: 20px;
}

#target {
  position: absolute;
  background: green;

  width: 80px;
  height: 20px;
  top: -10px;
  left: -10px;
}

#cover {
  position: absolute;
  background: green;

  width: 80px;
  height: 20px;
  top: 40px;
  left: 0px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
