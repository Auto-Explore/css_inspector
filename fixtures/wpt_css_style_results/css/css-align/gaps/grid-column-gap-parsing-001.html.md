# css/css-align/gaps/grid-column-gap-parsing-001.html

```json
{
  "format_version": 3,
  "file": "css/css-align/gaps/grid-column-gap-parsing-001.html"
}
```

## style[0]

```css

  .columnGapPx { grid-column-gap: 12px; }
  #columnGapEm { grid-column-gap: 2em; font: 10px/1 Monospace; }
  #columnGapVw { grid-column-gap: 2vw; }
  #columnGapPercent { grid-column-gap: 15%; }
  #columnGapCalc { grid-column-gap: calc(10px + 4px); }
  #columnGapCalcFixedPercent { grid-column-gap: calc(5px + 10%); }
  .columnGapInitial { grid-column-gap: initial; }
  .columnGapInherit { grid-column-gap: inherit; }

  #invalidColumnGapNegative { grid-column-gap: -10px; }
  #invalidColumnGapMaxContent { grid-column-gap: max-content; }
  #invalidColumnGapNone { grid-column-gap: none; }
  #invalidColumnGapMultiple { grid-column-gap: 10px 1px; }
  #invalidColumnGapAngle { grid-column-gap: 3rad; }
  #invalidColumnGapResolution { grid-column-gap: 2dpi; }
  #invalidColumnGapTime { grid-column-gap: 200ms; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
