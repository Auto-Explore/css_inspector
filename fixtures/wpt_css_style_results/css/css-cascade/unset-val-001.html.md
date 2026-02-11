# css/css-cascade/unset-val-001.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/unset-val-001.html"
}
```

## style[0]

```css

.square {
  width: 100px;
  height: 100px;
}
.under {
  background-color: green;
  margin-bottom: -100px;
}
.outer {
  color: green;
  background: red;
}
.inner {
  color: red;
  background-color: red;
  font-size: 40px;
  text-align: center;
}
.inner {
  color: unset;/* inherit from .outer */
  background-color: unset;/* initial, transparent, .under shows thru */
}
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
