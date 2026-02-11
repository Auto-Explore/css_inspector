# css/filter-effects/backdrop-filter-isolation-isolate.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/backdrop-filter-isolation-isolate.html"
}
```

## style[0]

```css

.box {
  position: absolute;
  width: 100px;
  height: 100px;
  background: green;
}
.outside {
  top: 110px;
  left: 10px;
}
.stacking-context {
  isolation: isolate;
  top: 0px;
  left: 120px;
}
.filter {
  width: 160px;
  height: 160px;
  top: 30px;
  left: -90px;
  backdrop-filter: invert(1);
  background: #ff08;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
