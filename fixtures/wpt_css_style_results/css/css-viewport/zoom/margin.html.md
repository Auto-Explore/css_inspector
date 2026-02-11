# css/css-viewport/zoom/margin.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/margin.html"
}
```

## style[0]

```css

#reference-overlapped-red {
  background-color: red;
  width: 100px;
  height: 100px;
  z-index: -1;
}
#reference-overlapped-big-red {
  background-color: red;
  width: 200px;
  height: 200px;
  z-index: -1;
}

.container {
  padding: 20px;
}
.margin-container {
  width: 200px;
  margin-left: -50px;
}
.margin-big-container {
  width: 400px;
  margin-left: -100px;
}
.green-box {
  background-color: green;
  width: 100px;
  height: 100px;
  display: flex;
  justify-content: center;
  align-items: center;
  text-align: center;
  margin: auto;
  margin-top: -100px;
  justify-self: right;
}
.zoom {
  zoom: 2;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
