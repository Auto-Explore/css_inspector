# css/css-anchor-position/anchor-center-overflow-004.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-center-overflow-004.html"
}
```

## style[0]

```css

  .container {
    width: 80px; height: 80px;
    border: solid gray;
    margin: 6px;
    position: relative;
    anchor-scope: --tl, --tr, --br, --bt;
    float: left;
  }

  .anchor {
    border: solid;
    margin: 10px;
    position: absolute;
  }
  .anchor.tl { anchor-name: --tl; top: 0; left: 0; }
  .anchor.tr { anchor-name: --tr; top: 0; right: 0; }
  .anchor.bl { anchor-name: --bl; bottom: 0; left: 0; }
  .anchor.br { anchor-name: --br; bottom: 0; right: 0; }

  .anchored {
    width: 40px; height: 40px;
    border: solid;
    position: absolute;
    place-self: anchor-center;
    margin: 2px;
    inset: -10px;
  }
  .anchored.tl { position-anchor: --tl; }
  .anchored.tr { position-anchor: --tr; }
  .anchored.bl { position-anchor: --bl; }
  .anchored.br { position-anchor: --br; }

  .tl { border-color: blue; }
  .tr { border-color: aqua; }
  .bl { border-color: fuchsia; }
  .br { border-color: orange; }

  body > div { clear: both; }
  .mix .anchored.tl { writing-mode: horizontal-tb; direction: rtl; }
  .mix .anchored.tr { writing-mode: vertical-rl; direction: ltr; }
  .mix .anchored.bl { writing-mode: vertical-lr; direciton: rtl; }
  .mix .anchored.br { writing-mode: sideways-lr; direction: ltr; }
```

```json
{
  "errors": 11,
  "messages": [
    {
      "message": "Unknown property “anchor-scope”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “direciton”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
