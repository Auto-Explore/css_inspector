# css/css-gaps/parsing/gap-decorations-bidirectional-shorthands.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/parsing/gap-decorations-bidirectional-shorthands.html"
}
```

## style[0]

```css

  #target1 {
    column-rule-color: lime;
    row-rule-color: lime;

    column-rule-width: 10px;
    row-rule-width: 10px;

    column-rule-style: solid;
    row-rule-style: solid;

    column-rule-break: intersection;
    row-rule-break: intersection;

    column-rule-visibility-items: around;
    row-rule-visibility-items: around;

    column-rule-edge-inset-start: 10px;
    row-rule-edge-inset-start: 10px;
    column-rule-edge-inset-end: 5px;
    row-rule-edge-inset-end: 5px;
    column-rule-interior-inset-start: 10px;
    row-rule-interior-inset-start: 10px;
    column-rule-interior-inset-end: 5px;
    row-rule-interior-inset-end: 5px;
  }

  #target2 {
    column-rule-color: blue;
    row-rule-color: red;

    column-rule-width: 15px;
    row-rule-width: 20px;

    column-rule-style: double;
    row-rule-style: dotted;

    column-rule-break: intersection;
    row-rule-break: normal;

    column-rule-visibility-items: around;
    row-rule-visibility-items: between;

    column-rule-edge-inset-start: 5px;
    row-rule-edge-inset-start: 10px;
    column-rule-edge-inset-end: 15px;
    row-rule-edge-inset-end: 20px;
    column-rule-interior-inset-start: 25px;
    row-rule-interior-inset-start: 30px;
    column-rule-interior-inset-end: 35px;
    row-rule-interior-inset-end: 40px;
  }
```

```json
{
  "errors": 30,
  "messages": [
    {
      "message": "Unknown property “row-rule-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-style”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-visibility-items”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-visibility-items”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-edge-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-edge-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-edge-inset-end”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-edge-inset-end”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-interior-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-interior-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-interior-inset-end”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-interior-inset-end”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-style”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-visibility-items”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-visibility-items”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-edge-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-edge-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-edge-inset-end”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-edge-inset-end”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-interior-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-interior-inset-start”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-interior-inset-end”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-interior-inset-end”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
