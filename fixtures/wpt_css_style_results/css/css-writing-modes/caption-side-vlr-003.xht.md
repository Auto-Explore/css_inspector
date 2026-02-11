# css/css-writing-modes/caption-side-vlr-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/caption-side-vlr-003.xht"
}
```

## style[0]

```css
<![CDATA[
  table#test-overlapping-green
    {
      border-spacing: 0px;
      caption-side: top;
      font: 50px/1 Ahem; /* computes to 50px/50px */
      writing-mode: vertical-lr;
    }

  caption
    {
      color: green;
    }

  td
    {
      color: transparent;
      padding: 0px;
    }

  div#reference-overlapped-red
    {
      background: url("support/pattern-rg-rg-100x100.png") no-repeat;
      bottom: 100px;
      height: 100px;
      position: relative;
      width: 100px;
      z-index: -1;
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
