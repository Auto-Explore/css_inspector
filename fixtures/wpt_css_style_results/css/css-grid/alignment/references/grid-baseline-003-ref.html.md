# css/css-grid/alignment/references/grid-baseline-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/references/grid-baseline-003-ref.html"
}
```

## style[0]

```css

.flexbox {
    display: flex;
}
.inline-flexbox {
    display: inline-flex;
}
.flex-one {
    flex: 1;
}
.inline-block { display: inline-block; }
.flexbox, .inline-flexbox { background-color: lightgrey; }
.border { border: 11px solid pink; }
.padding { padding: 13px; }
.margin { margin: 8px 0; }
.flexbox > div {
    min-width: 0;
    min-height: 0;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
