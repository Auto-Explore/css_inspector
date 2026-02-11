# css/css-writing-modes/table-progression-vrl-001.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/table-progression-vrl-001.html"
}
```

## style[0]

```css

  .test {
    writing-mode: vertical-rl;
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

  /* These rules must have no effect. */
  .test thead,
  .test tfoot,
  .test tbody,
  .test tr,
  .test td {
    writing-mode: horizontal-tb; /* For UAs not supporting vertical-lr */
    writing-mode: vertical-lr;
    direction: rtl;
  }
  .test[dir=rtl] thead,
  .test[dir=rtl] tfoot,
  .test[dir=rtl] tbody,
  .test[dir=rtl] tr,
  .test[dir=rtl] td {
    writing-mode: horizontal-tb;
    direction: ltr;
  }
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
