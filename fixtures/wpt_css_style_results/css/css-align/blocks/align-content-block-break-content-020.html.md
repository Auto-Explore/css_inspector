# css/css-align/blocks/align-content-block-break-content-020.html

```json
{
  "format_version": 3,
  "file": "css/css-align/blocks/align-content-block-break-content-020.html"
}
```

## style[0]

```css

  @import "/fonts/ahem.css";
  html { font: 10px/1 Ahem; max-width: 800px; }
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
    height: 19.5em;
    /* show bounds of test box without interfering with margin-collapsing */
    border-inline: solid black 1em;
  }
  .large, .float {
    height: 2.5em;
    break-inside: avoid;
    background: orange;
  }
  .float {
    float: right;
  }
  .margin {
    padding: 0.2em; /* using padding because of margin handling bugs
    margin-block: 0.5em;
  }
  .margin > * {
    margin-block: -0.3em;
switch back once fixed */
  }
  hr {
    border: solid 1px; margin: 0.5em; }

  /* readability */
  .pager { border: solid 2px gray; margin: 0.5em 0; }
  .test { color: #8888; text-align: center; }
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
