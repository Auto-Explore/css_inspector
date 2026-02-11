# css/css-grid/grid-lanes/tentative/grid-placement/grid-lanes-grid-placement-named-lines-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/grid-placement/grid-lanes-grid-placement-named-lines-002.html"
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
    grid-template-areas: "B A";
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
    grid-row-end:span 1!important;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “gap”.",
      "severity": "Error"
    },
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
