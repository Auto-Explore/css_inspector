# css/css-writing-modes/float-contiguous-vlr-013.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/float-contiguous-vlr-013.xht"
}
```

## style[0]

```css
<![CDATA[
  p
    {
      line-height: 1.25;
    }

  strong
    {
      line-height: 1;
    }

  div.floated-right
    {
      color: green;
      float: right;
      font: 20px/1 Ahem; /* computes to 20px/20px */
      writing-mode: vertical-lr;
    }

  div#reference-overlapped-red
    {
      background-color: red;
      height: 100px;
      position: absolute;
      right: 8px;
      top: 52px;
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
