# css/css-writing-modes/table-progression-vrl-003.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/table-progression-vrl-003.html"
}
```

## style[0]

```css

  .test {
    writing-mode: vertical-rl;
    text-orientation: upright;
  }
  [dir=rtl] {
    direction: rtl;
  }

  table {
    border-spacing: 0;
    margin: 1em;
  }
  td {
    width: 1em;
    height: 1em;
    border: solid gray;
  }

  .navy { background: navy}
  .blue { background: blue }
  .aqua { background: aqua }
  .teal { background: teal }
  .purp { background: purple }
  .pink { background: fuchsia }
  .yllw { background: yellow }
  .orng { background: orange }
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
