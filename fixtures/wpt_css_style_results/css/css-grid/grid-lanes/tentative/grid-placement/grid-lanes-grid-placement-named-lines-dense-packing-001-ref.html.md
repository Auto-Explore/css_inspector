# css/css-grid/grid-lanes/tentative/grid-placement/grid-lanes-grid-placement-named-lines-dense-packing-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/grid-placement/grid-lanes-grid-placement-named-lines-dense-packing-001-ref.html"
}
```

## style[0]

```css

body,html { color:black; background:white; font:15px/1 monospace; padding:0; margin:0; }

.grid {
  display: grid;
  grid-template-columns: 40px 40px 40px 60px 60px 60px 40px 40px 40px 40px 40px 40px 40px;
  position: relative;
  border: 1px solid;
  grid-auto-flow: dense;
  column-gap: 1px;
}

x {
  background: grey;
}
y {
  position: absolute;
  border: 1px solid blue;
  top:0;height:0;left:0;right:0;
  grid-row-end:span 1!important;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row-end”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
