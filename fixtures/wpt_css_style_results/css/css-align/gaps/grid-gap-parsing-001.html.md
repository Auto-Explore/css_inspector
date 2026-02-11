# css/css-align/gaps/grid-gap-parsing-001.html

```json
{
  "format_version": 3,
  "file": "css/css-align/gaps/grid-gap-parsing-001.html"
}
```

## style[0]

```css

  .gapPx { grid-gap: 12px; }
  #gapPxTwo { grid-gap: 12px 8px; }
  #gapPxPercent { grid-gap: 12px 10%; }
  #gapEm { grid-gap: 2em; font: 10px/1 Monospace; }
  #gapEmTwo { grid-gap: 2em 4em; font: 10px/1 Monospace; }
  #gapVw { grid-gap: 2vw; }
  #gapVwTwo { grid-gap: 2vw 1vh; }
  #gapPercent { grid-gap: 15%; }
  #gapPercentTwo { grid-gap: 15% 10%; }
  #gapCalc { grid-gap: calc(10px + 4px); }
  #gapCalcFixedPercent { grid-gap: calc(5px + 10%); }
  #gapCalcTwo { grid-gap: calc(10px + 4px) calc(20px - 8px); }
  .gapInitial { grid-gap: initial; }
  .gapInherit { grid-gap: inherit; }

  #invalidGridGapNegative { grid-gap: -10px; }
  #invalidGridGapMaxContent { grid-gap: max-content; }
  #invalidGridGapNone { grid-gap: none; }
  #invalidGridGapAngle { grid-gap: 3rad; }
  #invalidGridGapResolution { grid-gap: 2dpi; }
  #invalidGridGapTime { grid-gap: 200ms; }
  #invalidGridGapThree { grid-gap: 10px 1px 5px; }
  #invalidGridGapSlash { grid-gap: 10px / 5px; }
  #invalidGridGapOneWrong { grid-gap: 10px -5px; }
```

```json
{
  "errors": 9,
  "messages": [
    {
      "message": "Invalid value for property “grid-gap”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-gap”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-gap”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-gap”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-gap”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-gap”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-gap”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-gap”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-gap”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
