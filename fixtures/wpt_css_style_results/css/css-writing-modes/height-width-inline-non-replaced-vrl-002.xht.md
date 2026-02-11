# css/css-writing-modes/height-width-inline-non-replaced-vrl-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/height-width-inline-non-replaced-vrl-002.xht"
}
```

## style[0]

```css
<![CDATA[
  div#block
    {
      font: 100px/1 Ahem; /* computes to 100px/100px */
      height: 2em;
      width: 4em;
      writing-mode: vertical-rl;
    }

  div#inline
    {
      background-color: red;
      color: red;
      display: inline;
      height: 2em;
      width: 4em;
    }

  div#overlapping-green
    {
      background-color: green;
      bottom: 200px;
      height: 100px;
      left: 300px;
      position: relative;
      width: 100px;
    }
  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
