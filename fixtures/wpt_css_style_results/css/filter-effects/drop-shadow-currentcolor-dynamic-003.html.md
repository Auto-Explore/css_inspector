# css/filter-effects/drop-shadow-currentcolor-dynamic-003.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/drop-shadow-currentcolor-dynamic-003.html"
}
```

## style[0]

```css

#target {
  width: 100px;
  height: 100px;
  backdrop-filter: drop-shadow(50px 0px 0px currentcolor);
}
.red {
  color: red;
}
.green {
  color: green;
}
#container {
  /* 0.98 was chosen to work around https://bugzil.la/1915676 */
  opacity: 0.98;
  width: 50px;
  height: 100px;
  background-color: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
