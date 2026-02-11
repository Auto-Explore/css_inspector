# css/css-multicol/multicol-nested-032.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-nested-032.html"
}
```

## style[0]

```css

.multicol {
  column-count: 2;
  column-fill: auto;
  column-gap: 0;
}
#outer {
  width: 100px;
  height: 100px;
  background: red;
}
#nested {
  width: 100%;
  height: 200px;
}
#abscb {
  position: relative;
  width: 100%;

  /* Use a height less than the outer column container's height 100px to trigger
     the bug. */
  height: 50px;
}
#abspos {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 400px;
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
