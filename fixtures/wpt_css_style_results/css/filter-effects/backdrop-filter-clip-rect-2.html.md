# css/filter-effects/backdrop-filter-clip-rect-2.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/backdrop-filter-clip-rect-2.html"
}
```

## style[0]

```css

.box {
  display: inline-block;
  box-sizing: border-box;
  width: 100px;
  height: 100px;
  border-radius: 10px 20px 30px 40px;
  border-width: 10px;
}
.no-bf > .box {
  background: black;
}
.bf > .box {
  border-color: transparent;
  backdrop-filter: invert(1);
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
