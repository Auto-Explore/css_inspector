# css/filter-effects/filter-region-negative-positioned-child-001.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filter-region-negative-positioned-child-001.html"
}
```

## style[0]

```css

.box {
  width: 100px;
  height: 100px;
  position: absolute;
}
.green {
  background-color: green;
}
.blue {
  background-color: blue;
}
.above {
  top: -100px;
}
.parent {
  filter: url(#f1);
  background-color: white;
  position: relative;
  top: 100px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
