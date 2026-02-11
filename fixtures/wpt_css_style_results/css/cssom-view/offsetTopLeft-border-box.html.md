# css/cssom-view/offsetTopLeft-border-box.html

```json
{
  "format_version": 3,
  "file": "css/cssom-view/offsetTopLeft-border-box.html"
}
```

## style[0]

```css


.container {
  position: relative;
  font: 20px/1 Ahem;
  width: 150px;
  height: 100px;
  padding: 2px 10px;
  border-width: 3px 6px;
  border-style: solid;
  box-sizing: border-box;
}

.target { background: grey; }
.hl { writing-mode:horizontal-tb; }
.vlr { writing-mode:vertical-lr; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
