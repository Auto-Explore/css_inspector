# css/css-writing-modes/line-box-height-vrl-008.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/line-box-height-vrl-008.xht"
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
      writing-mode: vertical-rl;
      text-orientation: sideways;
    }

  span#padding-left
    {
      padding-left: 1.5em;
    }

  span#padding-right
    {
      padding-right: 1.5em;
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
