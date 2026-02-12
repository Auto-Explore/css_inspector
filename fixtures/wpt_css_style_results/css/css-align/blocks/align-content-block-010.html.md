# css/css-align/blocks/align-content-block-010.html

```json
{
  "format_version": 3,
  "file": "css/css-align/blocks/align-content-block-010.html"
}
```

## style[0]

```css

  @import "/fonts/ahem.css";
  html { font: 10px/1 Ahem; max-width: 800px; }
  .label { display: none; }
  #manual { display: none; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

  .in-flow .float { height: 0; !important }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css

  html, body { margin: 0; padding: 0; }

  .test { height: 5em; margin: 0.5em 1em;
          /* show bounds of test box without interfering with margin-collapsing */
          background: black; padding-right: 2px; }
  /* ensure float is contained */
    .float { float: right; background: orange; height: 2em }
  /* ensure margin is contained */
    .in-flow { margin-top: 1em; background: orange }
  /* ensure relpos is ignored */
    .relpos { position: relative; top: -1.5em; }
  /* ensure abspos static position follows alignment */
    .wrapper { position: relative; }
    .abspos { position: absolute; right: 0; margin-top: -1.5em; }
  /* ensure overflow is not counted */
    .overflow { height: 0; }

  /* cram into 800x600 */
  html { max-height: 600px; columns: 3 }
  .wrapper { break-inside: avoid; border: solid 2px gray; }

  /* readability */
  .test > * { color: #8888; }
  .label { color: black; font-weight: bold; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
