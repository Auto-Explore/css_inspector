# css/css-writing-modes/margin-collapse-vlr-023.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/margin-collapse-vlr-023.xht"
}
```

## style[0]

```css
<![CDATA[
  div#test
    {
      border: teal solid 1em;
      font: 25px/1 Ahem;
      height: 4em; /* computes to 100px */
      margin-bottom: 0.2em; /* computes to 5px */
      writing-mode: vertical-lr;
    }

  div#wrapper
    {
      background-color: orange;
      border-left: blue solid 1em;
      width: 3em;
    }

  div#wrapper > div
    {
      height: 4em;
      margin-left: 1em;
    }

  div#abs-pos
    {
      background-color: blue;
      position: absolute;
      top: 33px;  /* body's margin + div#test's border-top */
      width: 1em;
    }
  ]]>
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
