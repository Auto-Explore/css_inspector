# css/CSS2/floats/floats-wrap-bfc-with-margin-010.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats/floats-wrap-bfc-with-margin-010.html"
}
```

## style[0]

```css

.wrapper {
  float: left;
  width: 100px;
  border: 5px solid;
  margin: 75px 10px;
}
.float {
  float: left;
  width: 50px;
  height: 50px;
  background: cyan;
}
.bfc {
  overflow: hidden;
  height: 50px;
  background: green;
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
