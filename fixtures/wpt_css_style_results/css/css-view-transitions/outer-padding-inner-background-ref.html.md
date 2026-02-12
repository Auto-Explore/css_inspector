# css/css-view-transitions/outer-padding-inner-background-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/outer-padding-inner-background-ref.html"
}
```

## style[0]

```css

:root  { background: rebeccapurple; }
.target {
  width: 200px;
  height: 200px;
  contain: paint;
  view-transition-name: target;
  padding: 20px;
}

.child {
  width: 100px;
  height: 200px;
  position: relative;
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
