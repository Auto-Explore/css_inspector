# css/css-writing-modes/float-shrink-to-fit-vlr-009.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/float-shrink-to-fit-vlr-009.xht"
}
```

## style[0]

```css
<![CDATA[
  div#test-floated-left
    {
      color: green;
      float: left;
      font: 50px/1 Ahem; /* computes to 50px/50px */
      height: auto;
      writing-mode: vertical-lr;
    }

  div.inner-green-border-bottom
    {
      border-bottom: green solid 1em;
    }

  div#reference-red-overlapped
    {
      background-color: red;
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
