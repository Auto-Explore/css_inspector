# css/css-grid/grid-lanes/tentative/grid-placement/grid-lanes-grid-placement-named-lines-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/grid-placement/grid-lanes-grid-placement-named-lines-001.html"
}
```

## style[0]

```css

body,html { color:black; background:white; font-size:15px/1 monospace; padding:0; margin:0; }

.grid {
  display: grid-lanes;
  position: relative;
  border: 1px solid;
  grid-template-columns: [A-start] 60px 60px 60px;
  grid-template-areas: "B A";
  grid-auto-columns: 40px;
  grid-gap: 1px;
}

x {
  background: grey;
  border: 1px solid;
}
y {
  position: absolute;
  border-top: 1px solid blue;
  height:0;left:0;right:0;
}

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
