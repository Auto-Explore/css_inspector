# css/css-writing-modes/clearance-calculations-vrl-008.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/clearance-calculations-vrl-008.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      height: 116px;
      writing-mode: vertical-rl;
    }

  div#preceding-sibling
    {
      margin-left: 25px;
      width: 25px;
    }

  div#floated-left
    {
      float: left;
      width: 50px;
    }

  div#clearing-left
    {
      background-color: green;
      clear: left;
      margin-left: 8px;
      margin-right: 75px;
      width: 100px;
    }

  div#reference-overlapped-red
    {
      background-color: red;
      height: 100px;
      position: absolute;
      right: 108px;
      top: 8px;
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
