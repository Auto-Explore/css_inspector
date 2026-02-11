# css/filter-effects/reference/filter-region-negative-positioned-child-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/reference/filter-region-negative-positioned-child-001-ref.html"
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
.below {
  top: 100px;
}
.parent {
  filter: url(#f1);
  background-color: white;
  position: relative;
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
