# css/css-writing-modes/sizing-orthog-htb-in-vlr-019.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-htb-in-vlr-019.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-lr;
    }

  body
    {
      font-size: 16px;
      line-height: 1.25; /* therefore, each line box is 20px tall */
      margin-left: 0px;
      margin-right: 0px;
    }

  div#sized-400px-vlr-containing-block
    {
      width: 400px;
    }

  div#ortho-block-htb
    {
      border: blue solid 3px;
      width: auto;
      writing-mode: horizontal-tb;
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
