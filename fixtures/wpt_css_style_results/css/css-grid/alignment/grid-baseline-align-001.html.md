# css/css-grid/alignment/grid-baseline-align-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-baseline-align-001.html"
}
```

## style[0]

```css

  @import "/fonts/ahem.css";
  .grid {
    border: solid silver;
    margin: 1em 2px;
    font: 20px/1 Ahem;

    display: inline-grid;
    vertical-align: top;
    grid-template-columns: repeat(4, max-content);
  }
  .grid > div {
    border: black 10px;
    border-style: solid none;
    color: orange;
  }
  div + div {
    font-size: 2em;
  }
  div + div + div {
    font-size: 50%;
  }
  .self > div {
    align-self: baseline;
  }
  .content > div {
    align-content: baseline;
  }
  div.stretch {
    align-self: stretch;
  }

  .ref {
    position: relative;
    width: 80px;
    height: 96px;
  }
  .ref > div {
    position: absolute;
  }
  .ref1 { top:   16px; }
  .ref2 { left:  20px; }
  .ref3 { top:   24px;
          left:  60px; }
  .ref4 { right:  0px;
          top:    24px;
          bottom: 0px;
          width: 10px; }

  .ref.content > div:not(.stripe) {
    border-color: transparent;
  }
  .stripe {
    width: 80px;
    height: 76px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
