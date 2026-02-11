# css/css-writing-modes/sizing-orthog-prct-vlr-in-htb-004-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-prct-vlr-in-htb-004-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      height: 400px;
      padding: 100px 0px;
      writing-mode: vertical-lr;
    }

  body
    {
      font-size: 16px;
      line-height: 1.25; /* therefore, each line box is 20px tall */
    }

  div
    {
      border: blue solid 3px;
      height: 200px;
      left: 8px;
      position: absolute;
      top: 100px;
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
