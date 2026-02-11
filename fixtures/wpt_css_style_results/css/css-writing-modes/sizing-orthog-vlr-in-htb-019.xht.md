# css/css-writing-modes/sizing-orthog-vlr-in-htb-019.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-vlr-in-htb-019.xht"
}
```

## style[0]

```css
<![CDATA[
  body
    {
      font-size: 16px;
      line-height: 1.25; /* therefore, each line box is 20px tall */
      margin-bottom: 0px;
      margin-top: 0px;
    }

  div#sized-400px-htb-containing-block
    {
      height: 400px;
    }

  div#ortho-block-vlr
    {
      border: blue solid 3px;
      height: auto;
      writing-mode: vertical-lr;
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
