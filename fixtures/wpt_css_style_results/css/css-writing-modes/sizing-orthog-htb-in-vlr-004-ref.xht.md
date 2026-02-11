# css/css-writing-modes/sizing-orthog-htb-in-vlr-004-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-htb-in-vlr-004-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      width: calc(100px + 100vw + 100px);
    }

  body
    {
      font-size: 16px;
      line-height: 1.25; /* therefore, each line box is 20px tall */
      margin: 8px 0px;
    }

  div
    {
      border: blue solid 3px;
      box-sizing: border-box;
      margin: 0px 100px;
      width: 100vw;
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
