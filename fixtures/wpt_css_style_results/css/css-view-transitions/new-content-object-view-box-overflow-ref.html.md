# css/css-view-transitions/new-content-object-view-box-overflow-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/new-content-object-view-box-overflow-ref.html"
}
```

## style[0]

```css

.target {
  color: red;
  width: 100px;
  height: 100px;
  contain: paint;
  position: relative;
  top: 50px;
  left: 50px;
}
.child {
  width: 123px;
  height: 150px;
  background: lightblue;
  position: relative;
  top: -10px;
  left: -20px;
}
.grandchild {
  width: 25px;
  height: 25px;
  position: relative;
  top: 20px;
  left: 40px;
  background: green;
}
body { background: lightpink; }
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
