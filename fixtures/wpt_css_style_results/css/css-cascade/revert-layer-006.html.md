# css/css-cascade/revert-layer-006.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/revert-layer-006.html"
}
```

## style[0]

```css

#outer {
  background-color: red;
  width: 100px;
  height: 100px;
  overflow: hidden;
}
#inner {
  color: green;
  background-color: green;
  display: inline;
  display: revert-layer; /* This should behave as 'revert', setting 'display' to 'block' */
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
