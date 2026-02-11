# css/css-writing-modes/sizing-orthog-vlr-in-htb-007-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-vlr-in-htb-007-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      padding: 100px 0px;
      writing-mode: vertical-lr;
    }

  body
    {
      font-size: 16px;
      line-height: 1.25; /* therefore, each line box is 20px tall */
    }

  p
    {
      left: 8px;
      position: absolute;
      writing-mode: horizontal-tb;
    }

  p#sentence-before
    {
      margin-top: 0px;
      top: 100px;
    }

  div
    {
      border: blue solid 3px;
      box-sizing: border-box;
      height: 400px;
      left: 8px;
      position: absolute;
      top: 136px;
    }

  p#sentence-after
    {
      padding-bottom: 28px; /* 600px - 536px -52px + 16px ==  28px */
      top: 536px; /* 100px - 16px + 52px + 400px == 536px */
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
