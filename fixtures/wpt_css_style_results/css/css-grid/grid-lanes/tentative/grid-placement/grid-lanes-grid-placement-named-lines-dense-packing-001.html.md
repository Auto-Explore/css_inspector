# css/css-grid/grid-lanes/tentative/grid-placement/grid-lanes-grid-placement-named-lines-dense-packing-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/grid-placement/grid-lanes-grid-placement-named-lines-dense-packing-001.html"
}
```

## style[0]

```css

body,html { color:black; background:white; font:15px/1 monospace; padding:0; margin:0; }

.grid {
  display: grid-lanes;
  position: relative;
  border: 1px solid;
  grid-template-columns: [A-start] 60px 60px 60px;
  /* This creates implicit B-start and B-end lines, and A-start and E-end lines. */
  grid-template-areas: "B A";
  grid-auto-flow: dense;
  grid-lanes-pack: dense;
  grid-auto-columns: 40px;
  gap: 0 1px;
}

x {
  background: grey;
}

y {
  position: absolute;
  border: 1px solid blue;
  bottom:0;height:0;left:0;right:0;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “grid-lanes-pack”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “gap”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
