# css/css-writing-modes/sizing-orthog-prct-htb-in-vlr-004.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-prct-htb-in-vlr-004.xht"
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
      margin-left: 100px;
      margin-right: 100px;
    }

  div#sized-400px-vlr-containing-block
    {
      width: 400px;
    }

  div#ortho-block-htb
    {
      border: blue solid 3px;
      width: 50%;
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
