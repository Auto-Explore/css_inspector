# css/css-grid/grid-lanes/tentative/baseline/grid-lanes-grid-item-content-baseline-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/baseline/grid-lanes-grid-item-content-baseline-001.html"
}
```

## style[0]

```css

html,body {
    color:black; background-color:white; font:15px/1 Ahem; padding:0; margin:0;
}

.grid {
  float: left;
  display: grid-lanes;
  grid: none / repeat(4, auto);
  border: 2px solid;
  padding: 1px;
  margin: 1px;
}
.c {
  grid: repeat(4, auto) / none;
  grid-lanes-direction: row;
}

span {
  background: lime;
  border: 1px solid black;
}

span:nth-child(1) { font-size:15px; }
span:nth-child(2) { font-size:30px; }
span:nth-child(3) { font-size:10px; }
.pbs { padding-block-start: 15px; margin-block-start: 5px; }
.pbe { padding-block-end: 7px; margin-block-end: 3px; }

.fb { align-content:baseline; }
.lb { align-content:last baseline; }

.hl { writing-mode: horizontal-tb; direction:ltr; }
.hr { writing-mode: horizontal-tb; direction:rtl; }
.vl { writing-mode: vertical-lr; text-orientation: sideways; }
.vr { writing-mode: vertical-rl; text-orientation: sideways; }
.vlr { writing-mode: vertical-lr; direction:rtl; text-orientation: sideways; }
.vrl { writing-mode: vertical-rl; direction:ltr; text-orientation: sideways; }

```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
