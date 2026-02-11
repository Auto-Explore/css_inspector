# css/css-align/blocks/align-content-block-break-overflow-010.html

```json
{
  "format_version": 3,
  "file": "css/css-align/blocks/align-content-block-break-overflow-010.html"
}
```

## style[0]

```css

  @import "/fonts/ahem.css";
  html { font: 10px/1 Ahem; max-width: 780px; }
```

```json
{
  "errors": 0,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    }
  ],
  "warnings": 1
}
```

## style[1]

```css

  .pager {
    column-fill: auto;
    column-width: 15em;
    height: 5em;
  }
  .test {
    height: 19em;
    /* show bounds of test box without interfering with margin-collapsing */
    border-inline: solid black 1em;
  }
  .overflow {
    height: 0;
    /* show bounds of overflow box without interfering with margin-collapsing */
    background: red;
  }
  .large, .float {
    height: 2.5em;
    break-inside: avoid;
    padding: 2px; /* using padding because of margin handling bugs
    margin: 2px;
switch back once fixed */
    background: orange;
  }
  .float {
    float: right;
  }
  .nobr {
    break-inside: avoid;
  }

  /* readability */
  .pager { border: solid 2px gray; margin: 0.5em 0; }
  .test  { color: #8888; text-align: center; }
  .label { color: black; font-weight: bold; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border-inline”.",
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
