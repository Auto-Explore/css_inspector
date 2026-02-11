# css/filter-effects/backdrop-filter-clip-rect-ref.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/backdrop-filter-clip-rect-ref.html"
}
```

## style[0]

```css

div {
  position: absolute;
}
.box {
  width: 200px;
  height: 200px;
  top: 100px;
  left: 100px;
  background: green;
}
.navbar {
  width: 300px;
  height: 50px;
  top: 150px;
  left: 50px;
  border: 2px solid blue;
  backdrop-filter: invert(1);
  border-radius: 10px 20px 30px 40px;
}
.menu {
  width: 100px;
  height: 150px;
  top: 202px;
  left: 147px;
  border: 2px solid red;
}
.menu2 {
  width: 100px;
  height: 30px;
  top: 118px;
  left: 147px;
  border: 2px solid red;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-radius”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
