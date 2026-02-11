# css/filter-effects/backdrop-filter-zero-size.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/backdrop-filter-zero-size.html"
}
```

## style[0]

```css

div {
    position: absolute;
    width: 100px;
    height: 100px;
}
.colorbox {
    background: green;
    left: 10px;
    top: 100px;
}
.filterparent {
    width: 0px;
    height: 0px;
    left: 50px;
    top: 150px;
    backdrop-filter: invert(1);
    background: lime;
}
.filterchild {
    top: 100px;
    background: #FFFFFF00;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
