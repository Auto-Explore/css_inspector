# css/css-flexbox/percentage-heights-003.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/percentage-heights-003.html"
}
```

## style[0]

```css

.flexbox {
    display: flex;
    min-height: 100%;
}

.column {
    flex-flow: column;
}

.column-wrap {
    flex-flow: column wrap;
}

.flexbox > div {
    background: teal;
    flex-grow: 1;
}

.flexbox span {
    height: 100%;
    background: orange;
    display: block;
}
.vert-wm {
    writing-mode: vertical-rl;
}
.horiz-wm {
    writing-mode: horizontal-tb;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “flex-flow”.",
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
