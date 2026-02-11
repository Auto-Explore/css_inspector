# css/css-flexbox/flexbox-definite-sizes-004.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-definite-sizes-004.html"
}
```

## style[0]

```css

body { overflow: hidden }

.outerFlex {
  display: flex;
  width: 100px;
  /* Implicit "align-items:stretch" */
}

.innerFlex {
  display: flex;
  width: 100px;
  background: red;

  /* This reveals if we miscalculate the height of our flex item: */
  align-items: flex-end;
}

.block {
  width: 100px;
  max-height: 100%;
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
