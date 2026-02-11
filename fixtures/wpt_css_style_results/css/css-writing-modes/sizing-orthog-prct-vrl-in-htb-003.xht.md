# css/css-writing-modes/sizing-orthog-prct-vrl-in-htb-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-prct-vrl-in-htb-003.xht"
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

  div#sized-400px-htb-containing-block
    {
      height: 400px;
    }

  div#ortho-block-vrl
    {
      border: blue solid 3px;
      height: 50%;
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
