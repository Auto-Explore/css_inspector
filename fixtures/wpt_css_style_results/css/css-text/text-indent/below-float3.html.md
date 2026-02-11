# css/css-text/text-indent/below-float3.html

```json
{
  "format_version": 3,
  "file": "css/css-text/text-indent/below-float3.html"
}
```

## style[0]

```css

.container {
  position: relative;
  width: 100px;
  height: 100px;
  font-family: Ahem;
  line-height: 50px;
  text-indent: 50px;
  color: green;
  background-color: red;
}

.abs_pos {
  position: absolute;
  top: 50px;
  width: 50px;
  height: 50px;
  background-color: green;
}

.float_box {
  float: left;
  width: 100px;
  height: 50px;
  background-color: green;
}

.container::first-line {
  font-size: 50px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
