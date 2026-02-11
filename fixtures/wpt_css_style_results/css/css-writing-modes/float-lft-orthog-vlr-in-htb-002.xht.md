# css/css-writing-modes/float-lft-orthog-vlr-in-htb-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/float-lft-orthog-vlr-in-htb-002.xht"
}
```

## style[0]

```css
<![CDATA[
  div#horiz-parent
    {
      border: orange solid 8px;
      font-size: 32px;
      line-height: 1.25; /* computes to 40px */
    }

  div#orthog-vlr-float-left
    {
      background-color: blue;
      color: white;
      float: left;
      writing-mode: vertical-lr;
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
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
