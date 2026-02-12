# css/css-ui/animation/outline-offset-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/animation/outline-offset-interpolation.html"
}
```

## style[0]

```css

.parent {
  outline: solid 0px;
  outline-offset: 30px;
}
.target {
  width: 50px;
  height: 50px;
  background-color: black;
  display: inline-block;
  margin: 40px 0px 0px 40px;
  outline: 4px solid orange;
  outline-offset: 10px;
}
.expected {
  background-color: green;
  margin-right: 18px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
