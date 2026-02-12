# css/css-writing-modes/border-conflict-element-vrl-012.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/border-conflict-element-vrl-012.xht"
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
      writing-mode: vertical-rl;
    }

  td
    {
      padding: 0px;
    }



  table#first
    {
      border-right: red solid 100px;
    }

  table#first td
    {
      border-right: green solid 100px;
    }



  table#second > tbody
    {
      border-right: red solid 100px;
    }

  table#second td
    {
      border-right: green solid 100px;
    }



  table#third > thead
    {
      border-right: red solid 100px;
    }

  table#third td
    {
      border-right: green solid 100px;
    }



  table#fourth > tfoot
    {
      border-right: red solid 100px;
    }

  table#fourth td
    {
      border-right: green solid 100px;
    }



  table#fifth tr
    {
      border-right: red solid 100px;
    }

  table#fifth td
    {
      border-right: green solid 100px;
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
