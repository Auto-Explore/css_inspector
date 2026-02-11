# css/css-writing-modes/float-shrink-to-fit-vlr-007.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/float-shrink-to-fit-vlr-007.xht"
}
```

## style[0]

```css
<![CDATA[
  div.floated-left
    {
      background-color: red;
      float: left;
      font: 100px/1 Ahem; /* computes to 100px/100px */
      writing-mode: vertical-lr;
    }

  div.left-border
    {
      border-left: red solid 1em;
    }

  div.right-border
    {
      border-right: red solid 1em;
    }

  div.left-padding
    {
      padding-left: 1em;
    }

  div.right-padding
    {
      padding-right: 1em;
    }

  div#reference-overlapped-green
    {
      background-color: green;
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
