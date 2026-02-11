# css/css-writing-modes/border-conflict-element-vlr-013.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/border-conflict-element-vlr-013.xht"
}
```

## style[0]

```css
<![CDATA[
  table
    {
      border-collapse: collapse;
      direction: ltr;
      height: 20px;
      writing-mode: vertical-lr;
    }

  td
    {
      padding: 0px;
    }



  table#first
    {
      border-left: red solid 100px;
    }

  table#first td
    {
      border-left: green solid 100px;
    }



  table#second > tbody
    {
      border-left: red solid 100px;
    }

  table#second td
    {
      border-left: green solid 100px;
    }



  table#third > thead
    {
      border-left: red solid 100px;
    }

  table#third td
    {
      border-left: green solid 100px;
    }



  table#fourth > tfoot
    {
      border-left: red solid 100px;
    }

  table#fourth td
    {
      border-left: green solid 100px;
    }



  table#fifth tr
    {
      border-left: red solid 100px;
    }

  table#fifth td
    {
      border-left: green solid 100px;
    }

  div#reference-overlapped-red
    {
      background-color: red;
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
