# css/filter-effects/filter-region-transformed-composited-child-001.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filter-region-transformed-composited-child-001.html"
}
```

## style[0]

```css

div {
    position: absolute;
}
.filtered {
    filter: url(#filter);
}
.child {
    background-color: gray;
    width: 50px;
    height: 50px;
    position: absolute;
    will-change: transform;
}
.p1 {
    left: 50px;
    top: 50px;
}
.c1 {
    transform: translate(0px, 25px);
}
.p2 {
    left: 150px;
    top: 50px;
}
.c2 {
    transform: translate(0px, -25px);
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
