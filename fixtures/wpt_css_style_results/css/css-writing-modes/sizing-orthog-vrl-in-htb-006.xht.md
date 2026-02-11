# css/css-writing-modes/sizing-orthog-vrl-in-htb-006.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-vrl-in-htb-006.xht"
}
```

## style[0]

```css
<![CDATA[
  body
    {
      font-size: 16px;
      line-height: 1.25; /* therefore, each line box is 20px tall */
      margin-bottom: 100px;
      margin-top: 100px;
      /* Nota bene: margin-top of p#sentence-before will collapse with body's margin-top */
    }

  div#auto-sized-htb-containing-block
    {
      height: auto;
      /*
      'height: auto' causes the measurement of the orthogonal
      box's containing block to be indefinite
      */
    }

  div#ortho-block-vrl
    {
      border: blue solid 3px;
      height: auto;
      writing-mode: vertical-rl;
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
