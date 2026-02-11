# css/filter-effects/backdrop-filter-basic-opacity-2.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/backdrop-filter-basic-opacity-2.html"
}
```

## style[0]

```css

.box {
    position: absolute;
    width: 100px;
    height: 100px;
    left: 10px;
    top: 100px;
}
.colorbox {
    background: green;
}
.filterbox {
    backdrop-filter: invert(1);
    opacity: 0.5;
    /* An invert backdrop-filter with opacity 0.5 should always give a grey
       result. It will always mix the background color with its inverse, in
       equal proportion.*/
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
