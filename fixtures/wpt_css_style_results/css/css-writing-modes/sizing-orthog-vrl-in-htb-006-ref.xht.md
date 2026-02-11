# css/css-writing-modes/sizing-orthog-vrl-in-htb-006-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-vrl-in-htb-006-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      padding: 100px 0px;
      writing-mode: vertical-rl;
    }

  body
    {
      font-size: 16px;
      line-height: 1.25; /* therefore, each line box is 20px tall */
    }

  div
    {
      border: blue solid 3px;
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
