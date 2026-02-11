# css/css-writing-modes/line-box-height-vlr-011.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/line-box-height-vlr-011.xht"
}
```

## style[0]

```css
<![CDATA[
  body
    {
      margin-left: 32px;
    }

  div
    {
      border: blue solid 2px;
      font-size: 32px;
      margin: 1em;
      writing-mode: vertical-lr;
      text-orientation: upright;
    }

  span#border-left
    {
      border-left: 1.5em solid transparent;
    }

  span#border-right
    {
      border-right: 1.5em solid transparent;
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
