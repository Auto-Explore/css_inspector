# css/css-writing-modes/contiguous-floated-table-vrl-006.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/contiguous-floated-table-vrl-006.xht"
}
```

## style[0]

```css
<![CDATA[
  table
    {
      border-collapse: separate;
      border-spacing: 0px;
      float: left;
      height: 100px;
      writing-mode: vertical-rl;
    }

  td
    {
      padding: 0px;
    }

  table#left-first
    {
      border-right: green solid 25px;
      margin-right: 25px;
    }

  table#right-second
    {
      border-left: green solid 25px;
      margin-left: 25px;
    }

  div#reference-overlapped
    {
      background-color: red;
      background-image: url("support/margin-collapse-2em-space-wm-vert.png");
      background-repeat: no-repeat;
      height: 100px;
      width: 100px;
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
