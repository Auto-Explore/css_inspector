# css/css-transitions/events-006.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/events-006.html"
}
```

## style[0]

```css

.before::before,
.after:after {
  content: '';
  transition: padding-left .01s;
  padding-left: 1px;
}
.before.active::before,
.after.active:after {
  padding-left: 10px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
